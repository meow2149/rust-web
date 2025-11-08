use tower::layer::util::Stack;
use tower_http::request_id::{MakeRequestUuid, PropagateRequestIdLayer, SetRequestIdLayer};

pub const REQUEST_ID_HEADER: &str = "x-request-id";

pub fn layer() -> Stack<PropagateRequestIdLayer, SetRequestIdLayer<MakeRequestUuid>> {
    let set_layer = SetRequestIdLayer::x_request_id(MakeRequestUuid);
    let propagate_layer = PropagateRequestIdLayer::x_request_id();

    Stack::new(propagate_layer, set_layer)
}
