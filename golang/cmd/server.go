package main

import (
	"context"
	"log"
	"net"
	simple "simple/pb"
	"time"

	"google.golang.org/grpc"
)

const (
	ServerAddress = "0.0.0.0:50052"
	ServerName    = "Golang Server"
)

type SimpleServer struct{}

func (*SimpleServer) WhereAmI(_ context.Context, req *simple.Address) (*simple.Location, error) {
	return &simple.Location{
		Name:       req.GetName(),
		ServerName: ServerName,
		Timestamp:  time.Now().Format(time.RFC3339),
	}, nil
}

func main() {
	srv := grpc.NewServer()
	var ss *SimpleServer

	simple.RegisterSimpleServiceServer(srv, ss)

	log.Println("Starting RPC server at", ServerAddress)

	l, err := net.Listen("tcp", ServerAddress)
	if err != nil {
		log.Fatalf("could not listen to %s: %v", ServerAddress, err)
	}

	log.Fatal(srv.Serve(l))

}
