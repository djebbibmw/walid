#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::{dispatch::DispatchResult, pallet_prelude::*};
    use frame_system::{self as system, pallet_prelude::*};

    // التعريف بالسمعة والعملات
    #[pallet::pallet]
    pub struct Pallet<T>(_);

    #[pallet::config]
    pub trait Config: frame_system::Config {}

    // تخزين السمعة للمستخدم
    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::call_index(0)]
        #[pallet::weight(10_000)]
        pub fn issue_token(origin: OriginFor<T>, amount: u32) -> DispatchResult {
            let who = ensure_signed(origin)?;

            // استرجاع السمعة الخاصة بالمستخدم
            let reputation = system::Pallet::<T>::account_nonce(&who);

            // سك العملة بناءً على السمعة
            let token_amount = reputation * amount;

            // إرسال التوكن للمستخدم
            T::Currency::deposit_creating(&who, token_amount.into());

            // إطلاق حدث سك العملة
            Self::deposit_event(Event::TokensIssued(who, token_amount));

            Ok(())
        }

        #[pallet::call_index(1)]
        #[pallet::weight(10_000)]
        pub fn transfer_tokens(origin: OriginFor<T>, to: T::AccountId, amount: u32) -> DispatchResult {
            let who = ensure_signed(origin)?;

            // التأكد من أن المستخدم لديه رصيد كافٍ
            T::Currency::transfer(&who, &to, amount.into(), frame_support::traits::ExistenceRequirement::AllowDeath)?;

            // إطلاق حدث تحويل التوكن
            Self::deposit_event(Event::TokensTransferred(who, to, amount));

            Ok(())
        }
    }

    // الأحداث التي تُسجل عند كل عملية
    #[pallet::event]
    #[pallet::generate_store(pub(super) trait Store)]
    pub enum Event<T: Config> {
        TokensIssued(T::AccountId, u32),
        TokensTransferred(T::AccountId, T::AccountId, u32),
    }
}
