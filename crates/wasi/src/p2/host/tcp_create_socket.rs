use crate::p2::bindings::{sockets::network::IpAddressFamily, sockets::tcp_create_socket};
use crate::p2::tcp::TcpSocket;
use crate::p2::{IoView, SocketResult, WasiImpl, WasiView, LogLevel};
use wasmtime::component::Resource;

impl<T> tcp_create_socket::Host for WasiImpl<T>
where
    T: WasiView,
{
    fn create_tcp_socket(
        &mut self,
        address_family: IpAddressFamily,
    ) -> SocketResult<Resource<TcpSocket>> {
        self.ctx().logger.log(LogLevel::DEBUG, "calling tcp create sockets (create_tcp_socket) function.".into());
        let socket = TcpSocket::new(address_family.into())?;
        let socket = self.table().push(socket)?;
        Ok(socket)
    }
}
