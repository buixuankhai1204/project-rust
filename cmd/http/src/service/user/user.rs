use bytes::BufMut;
use futures::TryStreamExt;
use opentelemetry::{global, trace::{Tracer, FutureExt, TraceContextExt}, Context};
use validator::Validate;
use warp::Reply;
// use fdglapi::business::account::prelude::*;
// use fdglapi::common::convert::{json_value_to_bool, json_value_to_i64};

use crate::model::user::login::LoginRequest;
use crate::model::default::*;
use crate::Result;
pub async fn login(
    request: LoginRequest
) -> Result<impl Reply> {

    let tracer = global::tracer("start_service");
    let span = tracer.start( format!("{}", "feedback"));
    let context = Context::current_with_span(span);

    let validtor = request.validate();
    if validtor.is_err() == true {
        let error = serde_json::to_value("error".to_string()).unwrap();

        let json_body = serde_json::from_value::<ErrorResponse>(error).unwrap();
        return Ok(warp::reply::json(&json_body));
    }
    // let json_data = business::fadovn::user::feedback().with_context(context).await;
    // if json_data.is_none() == true {
    //     let error_key = String::from(fdglapi::API_CAN_NOT_GET_FEEDBACK);
    //     let error_value = get_error_value(&error_key).unwrap();
    //     let json_body = serde_json::from_value::<ErrorResponse>(error_value).unwrap();
    //
    //     return Ok(warp::reply::json(&json_body));
    // }
    //
    // let json_data = json_data.unwrap();
    // let success = json_value_to_bool(&json_data, FADOVN_SUCCESS_KEY);
    // if success.is_none() == true  {
    //     let error_key = String::from(fdglapi::API_GET_FEEDBACK_EMPTY_FADOVN);
    //     let error_value = get_error_value(&error_key).unwrap();
    //     let json_body = serde_json::from_value::<ErrorResponse>(error_value).unwrap();
    //     return Ok(warp::reply::json(&json_body));
    // }
    //
    // let success = success.unwrap();
    // if success == false {
    //     let code = json_value_to_i64(&json_data, FADOVN_DATA_TOKEN_ERROR_CODE);
    //     if code.is_none() == false {
    //         let error_code = convert_fadovn_error_code(code.unwrap());
    //         if error_code.is_none() == false {
    //             let error_key = error_code.unwrap();
    //             let error_value = get_error_value(&error_key).unwrap();
    //             let json_body = serde_json::from_value::<ErrorResponse>(error_value).unwrap();
    //             return Ok(warp::reply::json(&json_body));
    //         }
    //     }
    //
    //     let error_key = String::from(fdglapi::API_CAN_NOT_GET_FEEDBACK_FADOVN);
    //     let error_value = get_error_value(&error_key).unwrap();
    //     let json_body = serde_json::from_value::<ErrorResponse>(error_value).unwrap();
    //     return Ok(warp::reply::json(&json_body));
    // }
    //
    // let data = json_data.get(FADOVN_DATA_KEY);
    // if data.is_none() == true {
    //     let error_key = String::from(fdglapi::API_DATA_GET_FEEDBACK_EMPTY_FADOVN);
    //     let error_value = get_error_value(&error_key).unwrap();
    //     let json_body = serde_json::from_value::<ErrorResponse>(error_value).unwrap();
    //     return Ok(warp::reply::json(&json_body));
    // }

    let json_body = BodyResponse::<LoginRequest> {
        code: 0,
        message: Some("SUCCESS_KEY".to_owned()),
        payload: Some(request),
    };
    Ok(warp::reply::json(&json_body))
}