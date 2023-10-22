// Fill out your copyright notice in the Description page of Project Settings.

#include "ArenaServerConnector.h"
// #include "Interfaces/IPv4/IPv4Address.h"
#include <Runtime/Core/Public/HAL/RunnableThread.h>
#include <Runtime/Core/Public/HAL/UnrealMemory.h>
#include <Runtime/Core/Public/Async/Async.h>
#include <Runtime/Sockets/Public/SocketTypes.h>
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
	
    InitArenaGameMode();
}

// Called every frame
void AArenaServerConnector::Tick(float DeltaTime)
{
	Super::Tick(DeltaTime);

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

                isRun = true;
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

void AArenaServerConnector::Start()
{
    CreateSocket();

    // Listen . . . 

    while (isRun)
    {
        uint32 pendingDataSize = 0;
        TArray<uint8> recvedData;

        int32 totalReadData = 0;

        Socket->SetNonBlocking(true);
        int32 _read;
        uint8 _temp;
        if (!Socket->Recv(&_temp, 1, _read, ESocketReceiveFlags::Peek))
        {
            isRun = false;
            continue;
        }

        Socket->SetNonBlocking(false);

        while (isRun)
        {
            if (!Socket->HasPendingData(pendingDataSize))
            {
                break;
            }

            recvedData.SetNumUninitialized(totalReadData + pendingDataSize);

            int32 readData = 0;

            if (!Socket->Recv(recvedData.GetData() + totalReadData, pendingDataSize, readData))
            {
                // Data Read Failed.
                break;
            }
            totalReadData = totalReadData + readData;
        }

        if (isRun && recvedData.Num() != 0)
        {
            // Recv Logic
            RecvMessageFromServer(recvedData);
        }

        // sleep for loop control . . .
    }

    // DisConnect
}

bool AArenaServerConnector::SendMessageToServer(const FString& Message)
{
    // 皋技瘤 傈价
    int32 BytesSent = 0;

    return Socket->Send((uint8*)TCHAR_TO_UTF8(*Message), Message.Len(), BytesSent);
}

void AArenaServerConnector::RecvMessageFromServer(TArray<uint8>& Message)
{
    if (arenaGameMode)
    {
        FString _data = ReadDataAsString(Message, Message.Num());
//        arenaGameMode->CallMessageFunctionByName();
//        arenaGameMode->CallMessageFunctionByUnique();
    }
}

void AArenaServerConnector::DisConnect()
{

}

void AArenaServerConnector::InitArenaGameMode()
{
    arenaGameMode = Cast<AArenaGameMode>(UGameplayStatics::GetGameMode(this));
}

FString AArenaServerConnector::ReadDataAsString(TArray<uint8>& Message, int32 length)
{
    if (length <= 0)
    {
        if (length < 0)
        {

        }
        return FString("");
    }
    if (Message.Num() < length)
    {
        return FString("");
    }

    TArray<uint8> StringAsArray;
    StringAsArray.Reserve(length);

    for (int i = 0; i < length; i++)
    {
        StringAsArray.Add(Message[0]);
        Message.RemoveAt(0);
    }

    std::string cstr(reinterpret_cast<const char*>(StringAsArray.GetData()), StringAsArray.Num());
    return FString(UTF8_TO_TCHAR(cstr.c_str()));
}

FArenaGameMessage AArenaServerConnector::ConvertDataToGameMessage(FString data)
{

    // sendMsg = "_command:_param"
    // ex) "0:1.0,2.2,300"

    FArenaGameMessage msg;

//    data.Split

    return msg;
}

TArray<float> AArenaServerConnector::ConvertDataToFuncParam(FString splitedData)
{
    TArray<float> _arr;

    return _arr;
}

