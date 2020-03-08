const net = require('net');
const clients = [];
(async () => {
  net.createServer(function (socket) {
    socket.name = socket.remoteAddress + ":" + socket.remotePort;
    clients.push(socket);
    socket.write("Welcome " + socket.name + "\n");
    broadcast(socket.name + " joined\n", socket);
    socket.on('data', function (data) {
      broadcast(socket.name + "> " + data, socket);
    });
    socket.on('end', function () {
      clients.splice(clients.indexOf(socket), 1);
      broadcast(socket.name + " left.\n");
    });
    function broadcast(message, sender) {
      clients.forEach(function (client) {
        if (client === sender) return;
        client.write(message);
      });
      process.stdout.write(message)
    }

  }).listen(4444, '0.0.0.0');
  console.log("running at port 4444\n");
})();