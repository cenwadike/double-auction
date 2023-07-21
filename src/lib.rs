//! # Auction
//!
//! ## Overview
//!
//! This module provides a basic implement for order-book style on-chain double auctioning.
//!
//! This is the matching layer of a decentralized marketplace for electrical energy.
//! Sellers are categorized based on how much electricity they intend to sell.
//! Buyers are also categorized based on how much electricity they intend to buy.
//!
//! The highest bidding buyer in the same category with a seller is matched
//!  when the auction period of a seller is over.
//!
//! The seller has the benefit of getting the best price at a given point in time for their category,
//! while the buyer can choose a margin of safety for every buy.
//!
//! NOTE: this mocdule does not implement how payment is handled.
//!
//! `Data`:     
//!     --  AuctionData {
//!             seller_id: AccountId,
//!             quantity: u128,
//!             starting_bid: u128,
//!             buyers: [], // sorted array of bidders according to bid. Highest bidder at the top of the array.
//!             auction_period: Blockheight,
//!             start_at: Blockheight,
//!             ended_at: Blockheight,
//!         }
//!     -- Tier: u128,  // 0, 1, 2, ...
//!     -- Auctions {map(hash(AuctionData + Salt) -> (AuctionData, AuctionCategory, Tier)}
//!
//! `Interface`:
//!     -- create_auction(...)
//!     -- bid_on_auction(...)
//!     -- destroy_auction(...)
//!
//! `Hooks`:
//!     -- on_auctions_created
//!     -- on_auction_destroyed
//!     -- on_bid_auction
//!     -- on_auction_over
//!
//! `RPC`: Data RPCs

#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/reference/frame-pallets/>
pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;
// pub mod weights;
// pub use weights::*;

#[frame_support::pallet]
pub mod pallet {

    use super::*;
    use frame_support::inherent::Vec;
    use frame_support::{pallet_prelude::*, Twox64Concat};
    use frame_system::pallet_prelude::*;

    #[pallet::pallet]
    #[pallet::without_storage_info]
    pub struct Pallet<T>(_);

    /// Configure the pallet by specifying the parameters and types on which it depends.
    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// Because this pallet emits events, it depends on the runtime's definition of an event.
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
        // /// Type representing the weight of this pallet
        // type WeightInfo: WeightInfo;
    }

    //////////////////////
    // Storage types   //
    /////////////////////

    // Buyers bid
    #[derive(Clone, Encode, Decode, Default, Eq, PartialEq, RuntimeDebug, TypeInfo)]
    pub struct Bid<AccountId> {
        buyer_id: AccountId,
        bid: u128,
    }

    // Status of an auction, live auctions accepts bids
    #[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo)]
    pub enum AuctionStatus {
        Alive,
        Dead,
    }
    impl Default for AuctionStatus {
        fn default() -> Self {
            AuctionStatus::Alive
        }
    }

    // Essential data for an auction
    #[derive(Clone, Encode, Decode, Default, Eq, PartialEq, RuntimeDebug, TypeInfo)]
    pub struct AuctionData<AccountId, BlockNumber, Bid, Tier> {
        pub seller_id: AccountId,
        pub quantity: u128,
        pub starting_bid: u128,
        bids: Vec<Bid>,
        auction_period: BlockNumber,
        auction_status: AuctionStatus,
        start_at: BlockNumber,
        ended_at: BlockNumber,
        highest_bid: Bid,
        auction_category: Tier,
    }

    // Tier of an auction sale
    // Higher quantity of energy for sale leads to higher tier
    #[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo)]
    pub struct Tier {
        pub level: u32,
    }
    impl Default for Tier {
        fn default() -> Self {
            Tier { level: 1 }
        }
    }

    //////////////////////
    // Storage item    //
    /////////////////////
    #[pallet::storage]
    #[pallet::getter(fn get_auction)]
    pub(super) type Auctions<T: Config> = StorageMap<
        _,
        Twox64Concat,
        T::Hash,
        AuctionData<T::AccountId, T::BlockNumber, Bid<T::AccountId>, Tier>,
        OptionQuery,
    >;

    //////////////////////
    // Runtime events  //
    /////////////////////
    // runtime event for important runtime actions
    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        AuctionCreated {
            seller: T::AccountId,
            energy_quantity: u128,
            starting_price: u128,
        },

        AuctionMatched {
            seller: T::AccountId,
            buyer: T::AccountId,
            energy_quantity: u128,
            starting_price: u128,
            highest_bid: u128,
            matched_at: T::BlockNumber,
        },

        AuctionExecuted {
            seller: T::AccountId,
            buyer: T::AccountId,
            energy_quantity: u128,
            starting_price: u128,
            highest_bid: u128,
            executed_at: T::BlockNumber,
        },

        AuctionDestroyed {
            seller: T::AccountId,
            energy_quantity: u128,
            starting_price: u128,
        },
    }

    //////////////////////
    // Pallet errors   //
    /////////////////////
    // Errors inform users that something went wrong.
    #[pallet::error]
    pub enum Error<T> {
        AuctionDoesNotExist,

        AuctionIsOver,

        UnAuthorizedCall,

        InsuffficientAttachedDeposit,
    }

    ///////////////////////////
    // Pallet extrinsics    //
    //////////////////////////
    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::call_index(0)]
        #[pallet::weight(100_000_000)]
        pub fn create_auction(_origin: OriginFor<T>) -> DispatchResult {
            Ok(())
        }
    }
}
