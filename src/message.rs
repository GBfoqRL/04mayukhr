//! Defines the `HttpMessage` trait that serves to encapsulate the operations of a single
//! request-response cycle on any HTTP connection.

use std::fmt::Debug;
use std::any::{Any, TypeId};
use std::io::{Read, Write};

use std::mem;

use typeable::Typeable;

use header::Headers;
use http::RawStatus;
use url::Url;

use method;
use version;
use traitobject;

/// Describes a request.
#[derive(Clone, Debug)]
pub struct RequestHead {
    /// The headers of the request
    pub headers: Headers,
    /// The method of the request
    pub method: method::Method,
    /// The URL of the request
    pub url: Url,
}

/// Describes a response.
#[derive(Clone, Debug)]
pub struct ResponseHead {
    /// The headers of the reponse
    pub headers: Headers,
    /// The raw status line of the response
    pub raw_status: RawStatus,
    /// The HTTP/2 version which generated the response
    pub version: version::HttpVersion,
}

/// The trait provides an API for sending an receiving HTTP messages.
pub trait HttpMessage: Write + Read + Send + Any + Typeable + Debug {
    /// Initiates a new outgoing request.
    ///
    /// Only the request's head is provided (in terms of the `RequestHead` struct).
    ///
    /// After this, the `HttpMessage` instance can be used as an `io::Write` in order to write the
    /// body of the request.
    fn set_outgoing(&mut self, head: RequestHead) -> ::Result<RequestHead>;
    /// Obtains the incoming response and returns its head (i.e. the `ResponseHead` struct)
    ///
    /// After this, the `HttpMessage` instance can be used as an `io::Read` in order to read out
    /// the response body.
    fn get_incoming(&mut self) -> ::Result<ResponseHead>;

    /// Closes the underlying HTTP connection.
    fn close_connection(&mut self) -> ::Result<()>;
}

impl HttpMessage {
    unsafe fn downcast_ref_unchecked<T: 'static>(&self) -> &T {
        mem::transmute(traitobject::data(self))
    }

    unsafe fn downcast_mut_unchecked<T: 'static>(&mut self) -> &mut T {
        mem::transmute(traitobject::data_mut(self))
    }

    unsafe fn downcast_unchecked<T: 'static>(self: Box<HttpMessage>) -> Box<T>  {
        let raw: *mut HttpMessage = mem::transmute(self);
        mem::transmute(traitobject::data_mut(raw))
    }
}

impl HttpMessage {
    /// Is the underlying type in this trait object a T?
    #[inline]
    pub fn is<T: Any>(&self) -> bool {
        (*self).get_type() == TypeId::of::<T>()
    }

    /// If the underlying type is T, get a reference to the contained data.
    #[inline]
    pub fn downcast_ref<T: Any>(&self) -> Option<&T> {
        if self.is::<T>() {
            Some(unsafe { self.downcast_ref_unchecked() })
        } else {
            None
        }
    }

    /// If the underlying type is T, get a mutable reference to the contained
    /// data.
    #[inline]
    pub fn downcast_mut<T: Any>(&mut self) -> Option<&mut T> {
        if self.is::<T>() {
            Some(unsafe { self.downcast_mut_unchecked() })
        } else {
            None
        }
    }

    /// If the underlying type is T, extract it.
    #[inline]
    pub fn downcast<T: Any>(self: Box<HttpMessage>)
            -> Result<Box<T>, Box<HttpMessage>> {
        if self.is::<T>() {
            Ok(unsafe { self.downcast_unchecked() })
        } else {
            Err(self)
        }
    }
}
