#include <iostream>
#include <string>
#include <cstring>
#include <unistd.h>
#include <arpa/inet.h>
#include <sys/socket.h>

struct Entity {
    float x;
    float y;
};

int main() {
    // Server information
    std::string serverIP = "127.0.0.1";
    int serverPort = 8080;

    // Create socket
    int sock = socket(AF_INET, SOCK_STREAM, 0);
    if (sock == -1) {
        std::cerr << "Failed to create socket!" << std::endl;
        return -1;
    }

    // Server address structure
    sockaddr_in serverAddr;
    serverAddr.sin_family = AF_INET;
    serverAddr.sin_port = htons(serverPort);
    inet_pton(AF_INET, serverIP.c_str(), &serverAddr.sin_addr);

    // Connect to server
    if (connect(sock, reinterpret_cast<sockaddr*>(&serverAddr), sizeof(serverAddr)) == -1) {
        std::cerr << "Failed to connect to server!" << std::endl;
        close(sock);
        return -1;
    }

    // Loop to maintain connection
    while (true) {
        // Input message from user
        std::string msg;
        std::cout << "Enter message (type 'exit' to quit): ";
        std::getline(std::cin, msg);

        // Check if user wants to exit
        if (msg == "exit") {
            break;
        }

        // Send message to server
        if (send(sock, msg.c_str(), msg.size(), 0) == -1) {
            std::cerr << "Failed to send message!" << std::endl;
            close(sock);
            return -1;
        }

        // Receive response from server
        char buf[4096];
        memset(buf, 0, sizeof(buf));
        int bytesReceived = recv(sock, buf, sizeof(buf), 0);
        if (bytesReceived == -1) {
            std::cerr << "Failed to receive response from server!" << std::endl;
            close(sock);
            return -1;
        }

        // Print received response
        std::cout << "Received message from server: " << std::string(buf, bytesReceived) << std::endl;
    }

    // Close socket
    close(sock);

    return 0;
}
