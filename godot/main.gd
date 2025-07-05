extends Node2D

const PORT = 9080

var tcp_server := TCPServer.new()
var peers: Dictionary = {}

func log_message(message: String) -> void:
	var time := "[color=#aaaaaa] %s |[/color] " % Time.get_time_string_from_system()
	%TextServer.text += time + message + "\n"

func _ready() -> void:
	if tcp_server.listen(PORT) != OK:
		log_message("Unable to start server.")
		set_process(false)

func _process(_delta: float) -> void:
	while tcp_server.is_connection_available():
		var conn: StreamPeerTCP = tcp_server.take_connection()
		if conn:
			var peer: WebSocketPeer = WebSocketPeer.new()
			if peer.accept_stream(conn) == OK:
				var id := conn.get_connected_host() + ":" + str(conn.get_connected_port())
				peers[id] = peer
				log_message("🔌 client connection: %s" % id)
			else:
				log_message("⚠️ Connection Failure")

	# Receive messages from each client
	for id: String in peers.keys():
		var peer: WebSocketPeer = peers[id]
		peer.poll()

		while peer.get_available_packet_count() > 0:
			var packet: PackedByteArray = peer.get_packet()
			var msg: String = packet.get_string_from_utf8()
			log_message("📨 [%s] %s" % [id, msg])

func _exit_tree() -> void:
	for id in peers:
		peers[id].close()
	tcp_server.stop()
