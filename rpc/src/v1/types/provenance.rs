// Copyright 2015-2017 Parity Technologies (UK) Ltd.
// This file is part of Parity.

// Parity is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Parity is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Parity.  If not, see <http://www.gnu.org/licenses/>.

//! Request Provenance

use std::fmt;
use ethcore::account_provider::DappId as EthDappId;
use v1::types::H256;

/// RPC request origin
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub enum Origin {
	/// RPC server (includes request origin)
	#[serde(rename="rpc")]
	Rpc(String),
	/// Dapps server (includes DappId)
	#[serde(rename="dapp")]
	Dapps(DappId),
	/// IPC server (includes session hash)
	#[serde(rename="ipc")]
	Ipc(H256),
	/// WS server (includes session hash)
	#[serde(rename="ws")]
	Ws(H256),
	/// Signer (includes session hash)
	#[serde(rename="signer")]
	Signer(H256),
	/// Unknown
	#[serde(rename="unknown")]
	Unknown,
}

impl Default for Origin {
	fn default() -> Self {
		Origin::Unknown
	}
}

impl fmt::Display for Origin {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match *self {
			Origin::Rpc(ref origin) => write!(f, "RPC (service: {})", origin),
			Origin::Dapps(ref origin) => write!(f, "Dapp {}", origin),
			Origin::Ipc(ref session) => write!(f, "IPC (session: {})", session),
			Origin::Ws(ref session) => write!(f, "WebSocket (session: {})", session),
			Origin::Signer(ref session) => write!(f, "UI (session: {})", session),
			Origin::Unknown => write!(f, "unknown origin"),
		}
	}
}

/// Dapplication Internal Id
#[derive(Debug, Default, Clone, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
pub struct DappId(pub String);

impl fmt::Display for DappId {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}", self.0)
	}
}

impl Into<String> for DappId {
	fn into(self) -> String {
		self.0
	}
}

impl From<String> for DappId {
	fn from(s: String) -> Self {
		DappId(s)
	}
}

impl<'a> From<&'a str> for DappId {
	fn from(s: &'a str) -> Self {
		DappId(s.to_owned())
	}
}

impl From<EthDappId> for DappId {
	fn from(id: EthDappId) -> Self {
		DappId(id.into())
	}
}

impl Into<EthDappId> for DappId {
	fn into(self) -> EthDappId {
		Into::<String>::into(self).into()
	}
}

#[cfg(test)]
mod tests {
	use serde_json;
	use super::{DappId, Origin};

	#[test]
	fn should_serialize_origin() {
		// given
		let o1 = Origin::Rpc("test service".into());
		let o2 = Origin::Dapps("http://parity.io".into());
		let o3 = Origin::Ipc(5.into());
		let o4 = Origin::Signer(10.into());
		let o5 = Origin::Unknown;
		let o6 = Origin::Ws(5.into());

		// when
		let res1 = serde_json::to_string(&o1).unwrap();
		let res2 = serde_json::to_string(&o2).unwrap();
		let res3 = serde_json::to_string(&o3).unwrap();
		let res4 = serde_json::to_string(&o4).unwrap();
		let res5 = serde_json::to_string(&o5).unwrap();
		let res6 = serde_json::to_string(&o6).unwrap();

		// then
		assert_eq!(res1, r#"{"rpc":"test service"}"#);
		assert_eq!(res2, r#"{"dapp":"http://parity.io"}"#);
		assert_eq!(res3, r#"{"ipc":"0x0000000000000000000000000000000000000000000000000000000000000005"}"#);
		assert_eq!(res4, r#"{"signer":"0x000000000000000000000000000000000000000000000000000000000000000a"}"#);
		assert_eq!(res5, r#""unknown""#);
		assert_eq!(res6, r#"{"ws":"0x0000000000000000000000000000000000000000000000000000000000000005"}"#);
	}

	#[test]
	fn should_serialize_dapp_id() {
		// given
		let id = DappId("testapp".into());

		// when
		let res = serde_json::to_string(&id).unwrap();

		// then
		assert_eq!(res, r#""testapp""#);
	}

	#[test]
	fn should_deserialize_dapp_id() {
		// given
		let id = r#""testapp""#;

		// when
		let res: DappId = serde_json::from_str(id).unwrap();

		// then
		assert_eq!(res, DappId("testapp".into()));
	}
}
