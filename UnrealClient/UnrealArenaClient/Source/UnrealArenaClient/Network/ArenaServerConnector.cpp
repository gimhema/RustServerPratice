// Fill out your copyright notice in the Description page of Project Settings.

#include "ArenaServerConnector.h"
// #include "Interfaces/IPv4/IPv4Address.h"
#include <Runtime/Networking/Public/Interfaces/IPv4/IPv4Address.h>
// #include <Interfaces/IPv4/IPv4Address.h>

// Sets default values
AArenaServerConnector::AArenaServerConnector()
{
 	// Set this actor to call Tick() every frame.  You can turn this off to improve performance if you don't need it.
	PrimaryActorTick.bCanEverTick = true;

}

// Called when the game starts or when spawned
void AArenaServerConnector::BeginPlay()
{
	Super::BeginPlay();
	
}

// Called every frame
void AArenaServerConnector::Tick(float DeltaTime)
{
	Super::Tick(DeltaTime);

}

void AArenaServerConnector::InitSocketSystem()
{

}

void AArenaServerConnector::CreateSocket()
{
    if (SocketSubsystemPtr)
    {
        // 家南 积己
//        TSharedRef<FInternetAddr> ServerAddress = SocketSubsystemPtr->CreateInternetAddr();
        bool bIsValid = true;
//        ServerAddress->SetIp(*GameServerIP, bIsValid);
//        ServerAddress->SetPort(GameServerPort);

        if (bIsValid)
        {
            // TCP 家南 积己
  //          Socket = SocketSubsystemPtr->CreateSocket(NAME_Stream, TEXT("YourSocketName"), false);

            // 辑滚俊 楷搬
            TSharedRef<FInternetAddr> ServerAddress = ISocketSubsystem::Get(PLATFORM_SOCKETSUBSYSTEM)->CreateInternetAddr();;

            FIPv4Address ip;
            FIPv4Address::Parse(GameServerIP, ip);

            ServerAddress->SetIp(ip.Value);
            ServerAddress->SetPort(GameServerPort);
            if (Socket->Connect(*ServerAddress))
            {
                // 皋技瘤 傈价
                int32 BytesSent = 0;
                bool bSuccess = SendMessageToServer("Hello RL Server . . . ");

                if (bSuccess)
                {
                    UE_LOG(LogTemp, Warning, TEXT("Send Message Successful"));
                }
                else
                {
                    UE_LOG(LogTemp, Error, TEXT("Failed Send Message"));
                }
            }
            else
            {
                UE_LOG(LogTemp, Error, TEXT("Connect Server Failed"));
            }
        }
        else
        {
            UE_LOG(LogTemp, Error, TEXT("This IP Address Invalid . . . "));
        }
    }
    else
    {
        UE_LOG(LogTemp, Error, TEXT("Failed Socket Subsystem"));
    }
}

bool AArenaServerConnector::SendMessageToServer(const FString& Message)
{
    // 皋技瘤 傈价
    int32 BytesSent = 0;

    return Socket->Send((uint8*)TCHAR_TO_UTF8(*Message), Message.Len(), BytesSent);
}

void AArenaServerConnector::RecvMessageFromServer()
{

}

void AArenaServerConnector::DisConnect()
{

}

