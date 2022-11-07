/// Interface for this contract to make XCC to the NEP-141 conpatible contracts.
pub mod ft;

/// Interface for this contract to make XCC to the NEP-171 compatible contracts.
pub mod nft;

/// This common request can be passed to certain methods in arguments.
pub mod request;

/// Interface for this contract to be called after receiveing FTs.
mod ft_receiver;
