//! EDS Models

/// Raw Model: what comes out of the XML before we do any processing
pub mod raw;

/// EDS Model: what we use to represent the CCSDS Blue Book EDS Specification (expression resolved and namespace qualified)
pub mod resolved;
