#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::{dispatch::DispatchResult, pallet_prelude::*};
    use frame_system::pallet_prelude::*;
    use frame_support::sp_runtime::traits::CheckedSub;
    use sp_std::vec::Vec;

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    #[pallet::config]
    pub trait Config: frame_system::Config {}

    // تخزين البيانات
    #[pallet::storage]
    #[pallet::getter(fn get_balance)]
    pub(super) type Balances<T: Config> = StorageMap<_, Twox64Concat, T::AccountId, u64, OptionQuery>;

    // تعريف الأحداث
    #[pallet::event]
    pub enum Event<T: Config> {
        BalanceSet(T::AccountId, u64),
        HelloMessage(T::AccountId),
    }

    // تنفيذ الدوال
    #[pallet::call]
    impl<T: Config> Pallet<T> {
        // دالة say_hello
        #[pallet::weight(10_000)]
        pub fn say_hello(origin: OriginFor<T>) -> DispatchResult {
            let sender = ensure_signed(origin)?;
            log::info!("تم تنفيذ نداء 'say_hello' من {:?}", sender);
            Self::deposit_event(Event::<T>::HelloMessage(sender));
            Ok(())
        }

        // دالة لتخزين رصيد المستخدم
        #[pallet::weight(10_000)]
        pub fn set_balance(origin: OriginFor<T>, balance: u64) -> DispatchResult {
            let sender = ensure_signed(origin)?;
            ensure!(balance >= 0, "الرصيد يجب أن يكون عددًا موجبًا");

            // تخزين الرصيد للمستخدم
            Balances::<T>::insert(sender.clone(), balance);
            log::info!("تم تحديث رصيد المستخدم: {:?}", sender);

            // نشر الحدث
            Self::deposit_event(Event::<T>::BalanceSet(sender, balance));

            Ok(())
        }

        // دالة لاسترجاع رصيد المستخدم
        #[pallet::weight(10_000)]
        pub fn get_balance(origin: OriginFor<T>) -> DispatchResult {
            let sender = ensure_signed(origin)?;
            match Balances::<T>::get(sender.clone()) {
                Some(balance) => log::info!("رصيد المستخدم {:?}: {:?}", sender, balance),
                None => log::info!("المستخدم {:?} ليس لديه رصيد", sender),
            }
            Ok(())
        }
    }

    // تخصيص الصلاحيات
    #[pallet::call]
    pub fn ensure_root(origin: OriginFor<T>) -> DispatchResult {
        let _who = ensure_root(origin)?;
        Ok(())
    }
}
