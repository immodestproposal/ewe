#![cfg_attr(not(feature = "std"), no_std)]
#![warn(missing_docs)]

// This is allowed across the board because matching on bool is more compact and easier to read than 
// if/else. Obviously this is subjective, but that is exactly why this lint should not exist.
//#![allow(clippy::match_bool)]

//! # Ewe: **E**rror-**W**rapping **E**xtensions
