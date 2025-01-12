#![cfg_attr(not(test), no_std)]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::{dispatch::DispatchResult, pallet_prelude::*};
    use frame_system::{pallet_prelude::*};
    
    #[pallet::pallet]
    pub struct Pallet<T>(_);

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::call_index(0)]
        #[pallet::weight(10_000)]
        pub fn send_message(
            origin: OriginFor<T>,
            msg_type: MessageType,
            content: Vec<u8>,
        ) -> DispatchResult {
            let sender = ensure_signed(origin)?;

            // Almacenar el mensaje en la cadena de bloques
            MessageQueue::<T>::mutate(|messages| {
                messages.push(Message {
                    sender: sender.clone(),
                    msg_type,
                    content,
                });
            });

            Ok(())
        }
    }

    // Tipos de mensajes posibles: Error, Alerta, Emergencia
    #[derive(Encode, Decode, Clone, PartialEq, Eq, Debug)]
    pub enum MessageType {
        Error,
        Alerta,
        Emergencia,
    }

    // Estructura del mensaje
    #[derive(Encode, Decode, Clone, PartialEq, Eq, Debug)]
    pub struct Message<AccountId> {
        pub sender: AccountId,
        pub msg_type: MessageType,
        pub content: Vec<u8>,
    }

    // Almacén de mensajes
    #[pallet::storage]
    pub(super) type MessageQueue<T> = StorageValue<_, Vec<Message<T::AccountId>>, ValueQuery>;
}