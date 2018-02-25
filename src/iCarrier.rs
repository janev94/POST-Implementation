// Trait for carriers to implement
// TODO: needs refactoring
//	-- DONE 1. Add Message events in carrier

extern crate iron;

//use remote::Remote;

use message::Message;
//use message::Point;
use iSendable::ISendable;
use iReceivable::IReceivable;

pub trait ICarrier {
	type Item; // Type of messages the carrier will work with
	type Transmitter;
	//fn init(&self, remote: Remote) -> Box<ICarrier>;

	fn data_recv<T>(received: T) -> Message<Self::Item>
		where T: IReceivable<Message<Self::Item>>;

	fn msg_recv(message: &Message<Self::Item> );
	//TODO: Default impl, find a way to restrict self to impl MessageHandler
	//{
	//	self.on_msg_recv(message);
	//}

//	fn on_msg_rcv(message: &Message<Self::Item>);

	fn send_msg<T>(&self, message: T) where T: ISendable<Self::Transmitter>;

}
