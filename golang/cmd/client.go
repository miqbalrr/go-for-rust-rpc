package main

import (
	"context"
	"fmt"
	"log"
	simple "simple/pb"

	"google.golang.org/grpc"
)

const (
	RustRPCServer = "0.0.0.0:50051"
)

func main() {
	serv := RustRPCServer
	conn, err := grpc.Dial(serv, grpc.WithInsecure())
	if err != nil {
		log.Fatal("could not connect to", serv, err)
	}

	ssc := simple.NewSimpleServiceClient(conn)

	loc, err := ssc.WhereAmI(context.TODO(), &simple.Address{
		Name: "golang-client",
	})

	fmt.Println(loc)
}
