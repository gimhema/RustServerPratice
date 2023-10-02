// Fill out your copyright notice in the Description page of Project Settings.


#include "ArenaServerConnector.h"

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
        // ���� ����
        TSharedRef<FInternetAddr> ServerAddress = SocketSubsystemPtr->CreateInternetAddr();
        bool bIsValid;
        ServerAddress->SetIp(*GameServerIP, bIsValid);
        ServerAddress->SetPort(GameServerPort);

        if (bIsValid)
        {
            // TCP ���� ����
            Socket = SocketSubsystemPtr->CreateSocket(NAME_Stream, TEXT("YourSocketName"), false);

            // ������ ����
            if (Socket->Connect(*ServerAddress))
            {
                // �޼��� ����
                int32 BytesSent = 0;
                bool bSuccess = SendMessageToRLServer("Hello RL Server . . . ");

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

bool AArenaServerConnector::SendMessageToRLServer(const FString& Message)
{
    // �޼��� ����
    int32 BytesSent = 0;

    return Socket->Send((uint8*)TCHAR_TO_UTF8(*Message), Message.Len(), BytesSent);
}

