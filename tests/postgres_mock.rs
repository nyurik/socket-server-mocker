//! database: mockeddatabase, user: admin, password: password
//! CREATE TABLE playground (
//! id serial PRIMARY KEY,
//! data1 varchar (50) NOT NULL,
//! data2 varchar (50) NOT NULL
//! );
//! Note: In modern PostgreSQL, the default authentication method is scram-sha-256.
//! This hash method is secured by a nonce, so this mocked server uses md5 instead.

use postgres::{Client, NoTls};
use socket_server_mocker::server_mocker_instruction::ServerMockerInstruction;
use socket_server_mocker::tcp_server_mocker::TcpServerMocker;

#[test]
fn postgres_insert_mock() {
    // Mock PostgreSQL server on a port 54321 (default PostgresSQL port is 5432)
    let postgres_server_mocker = TcpServerMocker::new(54321);

    // Add mock binary messages corresponding to client connection and authentication
    postgres_server_mocker.add_mock_instructions(&[
        ServerMockerInstruction::ReceiveMessage,
        ServerMockerInstruction::SendMessage(vec![
            0x52, 0x00, 0x00, 0x00, 0x0c, 0x00, 0x00, 0x00, 0x05, 0x1c, 0x53, 0xa5, 0xf3,
        ]),
        ServerMockerInstruction::ReceiveMessage,
        ServerMockerInstruction::SendMessage(vec![
            0x52, 0x00, 0x00, 0x00, 0x08, 0x00, 0x00, 0x00, 0x00, 0x53, 0x00, 0x00, 0x00, 0x16,
            0x61, 0x70, 0x70, 0x6c, 0x69, 0x63, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x5f, 0x6e, 0x61,
            0x6d, 0x65, 0x00, 0x00, 0x53, 0x00, 0x00, 0x00, 0x19, 0x63, 0x6c, 0x69, 0x65, 0x6e,
            0x74, 0x5f, 0x65, 0x6e, 0x63, 0x6f, 0x64, 0x69, 0x6e, 0x67, 0x00, 0x55, 0x54, 0x46,
            0x38, 0x00, 0x53, 0x00, 0x00, 0x00, 0x17, 0x44, 0x61, 0x74, 0x65, 0x53, 0x74, 0x79,
            0x6c, 0x65, 0x00, 0x49, 0x53, 0x4f, 0x2c, 0x20, 0x44, 0x4d, 0x59, 0x00, 0x53, 0x00,
            0x00, 0x00, 0x26, 0x64, 0x65, 0x66, 0x61, 0x75, 0x6c, 0x74, 0x5f, 0x74, 0x72, 0x61,
            0x6e, 0x73, 0x61, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x5f, 0x72, 0x65, 0x61, 0x64, 0x5f,
            0x6f, 0x6e, 0x6c, 0x79, 0x00, 0x6f, 0x66, 0x66, 0x00, 0x53, 0x00, 0x00, 0x00, 0x17,
            0x69, 0x6e, 0x5f, 0x68, 0x6f, 0x74, 0x5f, 0x73, 0x74, 0x61, 0x6e, 0x64, 0x62, 0x79,
            0x00, 0x6f, 0x66, 0x66, 0x00, 0x53, 0x00, 0x00, 0x00, 0x19, 0x69, 0x6e, 0x74, 0x65,
            0x67, 0x65, 0x72, 0x5f, 0x64, 0x61, 0x74, 0x65, 0x74, 0x69, 0x6d, 0x65, 0x73, 0x00,
            0x6f, 0x6e, 0x00, 0x53, 0x00, 0x00, 0x00, 0x1b, 0x49, 0x6e, 0x74, 0x65, 0x72, 0x76,
            0x61, 0x6c, 0x53, 0x74, 0x79, 0x6c, 0x65, 0x00, 0x70, 0x6f, 0x73, 0x74, 0x67, 0x72,
            0x65, 0x73, 0x00, 0x53, 0x00, 0x00, 0x00, 0x14, 0x69, 0x73, 0x5f, 0x73, 0x75, 0x70,
            0x65, 0x72, 0x75, 0x73, 0x65, 0x72, 0x00, 0x6f, 0x6e, 0x00, 0x53, 0x00, 0x00, 0x00,
            0x19, 0x73, 0x65, 0x72, 0x76, 0x65, 0x72, 0x5f, 0x65, 0x6e, 0x63, 0x6f, 0x64, 0x69,
            0x6e, 0x67, 0x00, 0x55, 0x54, 0x46, 0x38, 0x00, 0x53, 0x00, 0x00, 0x00, 0x34, 0x73,
            0x65, 0x72, 0x76, 0x65, 0x72, 0x5f, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x00,
            0x31, 0x34, 0x2e, 0x35, 0x20, 0x28, 0x55, 0x62, 0x75, 0x6e, 0x74, 0x75, 0x20, 0x31,
            0x34, 0x2e, 0x35, 0x2d, 0x31, 0x2e, 0x70, 0x67, 0x64, 0x67, 0x32, 0x32, 0x2e, 0x30,
            0x34, 0x2b, 0x31, 0x29, 0x00, 0x53, 0x00, 0x00, 0x00, 0x20, 0x73, 0x65, 0x73, 0x73,
            0x69, 0x6f, 0x6e, 0x5f, 0x61, 0x75, 0x74, 0x68, 0x6f, 0x72, 0x69, 0x7a, 0x61, 0x74,
            0x69, 0x6f, 0x6e, 0x00, 0x61, 0x64, 0x6d, 0x69, 0x6e, 0x00, 0x53, 0x00, 0x00, 0x00,
            0x23, 0x73, 0x74, 0x61, 0x6e, 0x64, 0x61, 0x72, 0x64, 0x5f, 0x63, 0x6f, 0x6e, 0x66,
            0x6f, 0x72, 0x6d, 0x69, 0x6e, 0x67, 0x5f, 0x73, 0x74, 0x72, 0x69, 0x6e, 0x67, 0x73,
            0x00, 0x6f, 0x6e, 0x00, 0x53, 0x00, 0x00, 0x00, 0x1a, 0x54, 0x69, 0x6d, 0x65, 0x5a,
            0x6f, 0x6e, 0x65, 0x00, 0x45, 0x75, 0x72, 0x6f, 0x70, 0x65, 0x2f, 0x50, 0x61, 0x72,
            0x69, 0x73, 0x00, 0x4b, 0x00, 0x00, 0x00, 0x0c, 0x00, 0x00, 0x0a, 0x04, 0x45, 0x45,
            0x04, 0xb9, 0x5a, 0x00, 0x00, 0x00, 0x05, 0x49,
        ]),
    ]);

    // Connect to local mocked PostgreSQL server
    let mut client = Client::connect(
        "host=localhost user=admin password=password dbname=mockeddatabase port=54321",
        NoTls,
    )
    .unwrap();

    // Check connection message sent by the client to mock server is correct
    assert_eq!(
        vec![
            0x00, 0x00, 0x00, 0x41, 0x00, 0x03, 0x00, 0x00, 0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74,
            0x5f, 0x65, 0x6e, 0x63, 0x6f, 0x64, 0x69, 0x6e, 0x67, 0x00, 0x55, 0x54, 0x46, 0x38,
            0x00, 0x75, 0x73, 0x65, 0x72, 0x00, 0x61, 0x64, 0x6d, 0x69, 0x6e, 0x00, 0x64, 0x61,
            0x74, 0x61, 0x62, 0x61, 0x73, 0x65, 0x00, 0x6d, 0x6f, 0x63, 0x6b, 0x65, 0x64, 0x64,
            0x61, 0x74, 0x61, 0x62, 0x61, 0x73, 0x65, 0x00, 0x00
        ],
        postgres_server_mocker.pop_received_message().unwrap()
    );

    // Cannot verify the authentication message sent by the client to mock server because it contains a random salt
    postgres_server_mocker.pop_received_message().unwrap();

    // Add mock instructions corresponding to the client INSERT query
    postgres_server_mocker.add_mock_instructions(&[
        ServerMockerInstruction::ReceiveMessage,
        ServerMockerInstruction::SendMessage(vec![
            0x31, 0x00, 0x00, 0x00, 0x04, 0x74, 0x00, 0x00, 0x00, 0x0e, 0x00, 0x02, 0x00, 0x00,
            0x04, 0x13, 0x00, 0x00, 0x04, 0x13, 0x6e, 0x00, 0x00, 0x00, 0x04, 0x5a, 0x00, 0x00,
            0x00, 0x05, 0x49,
        ]),
        ServerMockerInstruction::ReceiveMessage,
        ServerMockerInstruction::SendMessage(vec![
            0x32, 0x00, 0x00, 0x00, 0x04, 0x43, 0x00, 0x00, 0x00, 0x0f, 0x49, 0x4e, 0x53, 0x45,
            0x52, 0x54, 0x20, 0x30, 0x20, 0x31, 0x00, 0x5a, 0x00, 0x00, 0x00, 0x05, 0x49,
        ]),
    ]);

    // Execute the INSERT query
    client
        .execute(
            "INSERT INTO playground (data1, data2) VALUES ($1, $2)",
            &[&"test1", &"test2"],
        )
        .unwrap();
}

#[test]
fn postgres_select_mock() {
    // Mock PostgreSQL server on a random free port (default PostgresSQL port is 5432)
    let postgres_server_mocker = TcpServerMocker::new(0);

    // Add mock binary messages corresponding to client connection and authentication
    postgres_server_mocker.add_mock_instructions(&[
        ServerMockerInstruction::ReceiveMessage,
        ServerMockerInstruction::SendMessage(vec![
            0x52, 0x00, 0x00, 0x00, 0x0c, 0x00, 0x00, 0x00, 0x05, 0xb8, 0x28, 0x2f, 0xf6,
        ]),
        ServerMockerInstruction::ReceiveMessage,
        ServerMockerInstruction::SendMessage(vec![
            0x52, 0x00, 0x00, 0x00, 0x08, 0x00, 0x00, 0x00, 0x00, 0x53, 0x00, 0x00, 0x00, 0x16,
            0x61, 0x70, 0x70, 0x6c, 0x69, 0x63, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x5f, 0x6e, 0x61,
            0x6d, 0x65, 0x00, 0x00, 0x53, 0x00, 0x00, 0x00, 0x19, 0x63, 0x6c, 0x69, 0x65, 0x6e,
            0x74, 0x5f, 0x65, 0x6e, 0x63, 0x6f, 0x64, 0x69, 0x6e, 0x67, 0x00, 0x55, 0x54, 0x46,
            0x38, 0x00, 0x53, 0x00, 0x00, 0x00, 0x17, 0x44, 0x61, 0x74, 0x65, 0x53, 0x74, 0x79,
            0x6c, 0x65, 0x00, 0x49, 0x53, 0x4f, 0x2c, 0x20, 0x44, 0x4d, 0x59, 0x00, 0x53, 0x00,
            0x00, 0x00, 0x26, 0x64, 0x65, 0x66, 0x61, 0x75, 0x6c, 0x74, 0x5f, 0x74, 0x72, 0x61,
            0x6e, 0x73, 0x61, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x5f, 0x72, 0x65, 0x61, 0x64, 0x5f,
            0x6f, 0x6e, 0x6c, 0x79, 0x00, 0x6f, 0x66, 0x66, 0x00, 0x53, 0x00, 0x00, 0x00, 0x17,
            0x69, 0x6e, 0x5f, 0x68, 0x6f, 0x74, 0x5f, 0x73, 0x74, 0x61, 0x6e, 0x64, 0x62, 0x79,
            0x00, 0x6f, 0x66, 0x66, 0x00, 0x53, 0x00, 0x00, 0x00, 0x19, 0x69, 0x6e, 0x74, 0x65,
            0x67, 0x65, 0x72, 0x5f, 0x64, 0x61, 0x74, 0x65, 0x74, 0x69, 0x6d, 0x65, 0x73, 0x00,
            0x6f, 0x6e, 0x00, 0x53, 0x00, 0x00, 0x00, 0x1b, 0x49, 0x6e, 0x74, 0x65, 0x72, 0x76,
            0x61, 0x6c, 0x53, 0x74, 0x79, 0x6c, 0x65, 0x00, 0x70, 0x6f, 0x73, 0x74, 0x67, 0x72,
            0x65, 0x73, 0x00, 0x53, 0x00, 0x00, 0x00, 0x14, 0x69, 0x73, 0x5f, 0x73, 0x75, 0x70,
            0x65, 0x72, 0x75, 0x73, 0x65, 0x72, 0x00, 0x6f, 0x6e, 0x00, 0x53, 0x00, 0x00, 0x00,
            0x19, 0x73, 0x65, 0x72, 0x76, 0x65, 0x72, 0x5f, 0x65, 0x6e, 0x63, 0x6f, 0x64, 0x69,
            0x6e, 0x67, 0x00, 0x55, 0x54, 0x46, 0x38, 0x00, 0x53, 0x00, 0x00, 0x00, 0x34, 0x73,
            0x65, 0x72, 0x76, 0x65, 0x72, 0x5f, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x00,
            0x31, 0x34, 0x2e, 0x35, 0x20, 0x28, 0x55, 0x62, 0x75, 0x6e, 0x74, 0x75, 0x20, 0x31,
            0x34, 0x2e, 0x35, 0x2d, 0x31, 0x2e, 0x70, 0x67, 0x64, 0x67, 0x32, 0x32, 0x2e, 0x30,
            0x34, 0x2b, 0x31, 0x29, 0x00, 0x53, 0x00, 0x00, 0x00, 0x20, 0x73, 0x65, 0x73, 0x73,
            0x69, 0x6f, 0x6e, 0x5f, 0x61, 0x75, 0x74, 0x68, 0x6f, 0x72, 0x69, 0x7a, 0x61, 0x74,
            0x69, 0x6f, 0x6e, 0x00, 0x61, 0x64, 0x6d, 0x69, 0x6e, 0x00, 0x53, 0x00, 0x00, 0x00,
            0x23, 0x73, 0x74, 0x61, 0x6e, 0x64, 0x61, 0x72, 0x64, 0x5f, 0x63, 0x6f, 0x6e, 0x66,
            0x6f, 0x72, 0x6d, 0x69, 0x6e, 0x67, 0x5f, 0x73, 0x74, 0x72, 0x69, 0x6e, 0x67, 0x73,
            0x00, 0x6f, 0x6e, 0x00, 0x53, 0x00, 0x00, 0x00, 0x1a, 0x54, 0x69, 0x6d, 0x65, 0x5a,
            0x6f, 0x6e, 0x65, 0x00, 0x45, 0x75, 0x72, 0x6f, 0x70, 0x65, 0x2f, 0x50, 0x61, 0x72,
            0x69, 0x73, 0x00, 0x4b, 0x00, 0x00, 0x00, 0x0c, 0x00, 0x00, 0x0a, 0xb6, 0xe4, 0x6b,
            0x48, 0xa2, 0x5a, 0x00, 0x00, 0x00, 0x05, 0x49,
        ]),
    ]);

    // Connect to local mocked PostgreSQL server
    let mut client = Client::connect(
        &*format!(
            "host=localhost user=admin password=password dbname=mockeddatabase port={}",
            postgres_server_mocker.listening_port()
        ),
        NoTls,
    )
    .unwrap();

    // Check connection message sent by the client to mock server is correct
    assert_eq!(
        vec![
            0x00, 0x00, 0x00, 0x41, 0x00, 0x03, 0x00, 0x00, 0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74,
            0x5f, 0x65, 0x6e, 0x63, 0x6f, 0x64, 0x69, 0x6e, 0x67, 0x00, 0x55, 0x54, 0x46, 0x38,
            0x00, 0x75, 0x73, 0x65, 0x72, 0x00, 0x61, 0x64, 0x6d, 0x69, 0x6e, 0x00, 0x64, 0x61,
            0x74, 0x61, 0x62, 0x61, 0x73, 0x65, 0x00, 0x6d, 0x6f, 0x63, 0x6b, 0x65, 0x64, 0x64,
            0x61, 0x74, 0x61, 0x62, 0x61, 0x73, 0x65, 0x00, 0x00
        ],
        postgres_server_mocker.pop_received_message().unwrap()
    );

    // Cannot verify the authentication message sent by the client to mock server because it contains a random salt
    postgres_server_mocker.pop_received_message().unwrap();

    // Add mock instructions corresponding to the client SELECT query
    postgres_server_mocker.add_mock_instructions(&[
        ServerMockerInstruction::ReceiveMessage,
        ServerMockerInstruction::SendMessage(vec![
            0x31, 0x00, 0x00, 0x00, 0x04, 0x74, 0x00, 0x00, 0x00, 0x06, 0x00, 0x00, 0x54, 0x00,
            0x00, 0x00, 0x4b, 0x00, 0x03, 0x69, 0x64, 0x00, 0x00, 0x00, 0x40, 0x0a, 0x00, 0x01,
            0x00, 0x00, 0x00, 0x17, 0x00, 0x04, 0xff, 0xff, 0xff, 0xff, 0x00, 0x00, 0x64, 0x61,
            0x74, 0x61, 0x31, 0x00, 0x00, 0x00, 0x40, 0x0a, 0x00, 0x02, 0x00, 0x00, 0x04, 0x13,
            0xff, 0xff, 0x00, 0x00, 0x00, 0x36, 0x00, 0x00, 0x64, 0x61, 0x74, 0x61, 0x32, 0x00,
            0x00, 0x00, 0x40, 0x0a, 0x00, 0x03, 0x00, 0x00, 0x04, 0x13, 0xff, 0xff, 0x00, 0x00,
            0x00, 0x36, 0x00, 0x00, 0x5a, 0x00, 0x00, 0x00, 0x05, 0x49,
        ]),
        ServerMockerInstruction::ReceiveMessage,
        ServerMockerInstruction::SendMessage(vec![
            0x32, 0x00, 0x00, 0x00, 0x04, 0x44, 0x00, 0x00, 0x00, 0x20, 0x00, 0x03, 0x00, 0x00,
            0x00, 0x04, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x05, 0x74, 0x65, 0x73, 0x74,
            0x31, 0x00, 0x00, 0x00, 0x05, 0x74, 0x65, 0x73, 0x74, 0x32, 0x43, 0x00, 0x00, 0x00,
            0x0d, 0x53, 0x45, 0x4c, 0x45, 0x43, 0x54, 0x20, 0x31, 0x00, 0x5a, 0x00, 0x00, 0x00,
            0x05, 0x49,
        ]),
    ]);

    // Execute the client SELECT query
    let rows = client.query("SELECT * FROM playground", &[]).unwrap();

    // Check the SELECT query result
    assert_eq!(1, rows.len());
    assert_eq!(1, rows[0].get::<_, i32>("id"));
    assert_eq!("test1", rows[0].get::<_, String>("data1"));
    assert_eq!("test2", rows[0].get::<_, String>("data2"));
}