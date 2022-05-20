use anyhow::Result;
use bitstamp::model::CurrencyPair;
use futures::{Future, Stream, StreamExt};
use model::make_merged_market_depth;
use std::{net::SocketAddr, pin::Pin};
use tonic::transport::Server;

use api::{orderbook_aggregator_server::OrderbookAggregator, Summary};

pub mod api;

pub use binance::binance_stream;
pub use bitstamp::bitstamp_detail_market_depth_stream;

pub mod model;

/// Start the grpc server
pub async fn serve<S>(addr: SocketAddr, service: S) -> Result<()>
where
    S: OrderbookAggregator + Send + Sync + 'static,
{
    log::info!("Orderbook server listening on {:?}", addr);

    let service = api::orderbook_aggregator_server::OrderbookAggregatorServer::new(service)
        .send_gzip()
        .accept_gzip();

    Server::builder().add_service(service).serve(addr).await?;

    Ok(())
}

pub struct SummaryServer {
    instrument: CurrencyPair,
}

impl SummaryServer {
    pub fn new(instrument: CurrencyPair) -> Self {
        SummaryServer { instrument }
    }
}

impl OrderbookAggregator for SummaryServer {
    type BookSummaryStream =
        Pin<Box<dyn Stream<Item = Result<Summary, tonic::Status>> + Send + 'static>>;

    fn book_summary<'life0, 'async_trait>(
        &'life0 self,
        _request: tonic::Request<api::Empty>,
    ) -> core::pin::Pin<
        Box<
            dyn Future<Output = Result<tonic::Response<Self::BookSummaryStream>, tonic::Status>>
                + core::marker::Send
                + 'async_trait,
        >,
    >
    where
        'life0: 'async_trait,
        Self: 'async_trait,
    {
        Box::pin(get_summary_stream(self.instrument))
    }
}

async fn get_summary_stream(
    instrument: CurrencyPair,
) -> Result<tonic::Response<<SummaryServer as OrderbookAggregator>::BookSummaryStream>, tonic::Status>
{
    log::info!("Creating orderbook summary stream");
    // Create a stream of binance market depth results
    log::debug!("Creating binance stream");
    let binance_stream = binance_stream(&format!("{}", instrument))
        .await
        .map_err(|err| {
            log::error!("{err:?}");
            tonic::Status::internal("Internal error")
        })?
        .map(|result| {
            log::debug!("Got bitstamp reply: {:?}", result);
            result.map_err(|err| {
                log::warn!("Failed binance item: {:?}", err);
                tonic::Status::internal("Retrieving binance order-book")
            })
        });
    // bitstamp market depth results
    log::debug!("Creating bitstamp stream");
    let bitstamp_stream = bitstamp_detail_market_depth_stream(instrument)
        .await
        .map_err(|err| {
            log::error!("{err:?}");
            tonic::Status::internal("Internal error")
        })?
        .map(|result| {
            log::debug!("Got bitstamp reply: {:?}", result);
            result.map_err(|err| {
                log::warn!("Failed bitstamp item: {:?}", err);
                tonic::Status::internal("Internal error")
            })
        });
    // Zip them together and convert them into a merged market depth
    let stream = binance_stream
        .zip(bitstamp_stream)
        .map(|(binance_result, bitstream_result)| {
            binance_result.and_then(|binance_data| {
                bitstream_result
                    .map(|bitstream_data| make_merged_market_depth(binance_data, bitstream_data))
            })
        });
    Ok(tonic::Response::new(Box::pin(stream)))
}
