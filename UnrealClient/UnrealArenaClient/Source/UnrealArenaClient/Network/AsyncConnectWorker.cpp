// Fill out your copyright notice in the Description page of Project Settings.


#include "AsyncConnectWorker.h"

#include <Runtime/Core/Public/HAL/RunnableThread.h>
#include <Runtime/Core/Public/HAL/UnrealMemory.h>
#include <Runtime/Core/Public/Async/Async.h>
#include <Runtime/Sockets/Public/SocketTypes.h>
#include <Runtime/Networking/Public/Interfaces/IPv4/IPv4Address.h>

#pragma region Main Thread Code
// This code will be run on the thread that invoked this thread (i.e. game thread)


FAsyncConnectWorker::FAsyncConnectWorker(/* You can pass in inputs here */)
{
	// Constructs the actual thread object. It will begin execution immediately
	// If you've passed in any inputs, set them up before calling this.
    UE_LOG(LogTemp, Warning, TEXT("Create AsyncConnectWorker . . . ."));
	Thread = FRunnableThread::Create(this, TEXT("Give your thread a good name"));
}


FAsyncConnectWorker::~FAsyncConnectWorker()
{
	if (Thread)
	{
		// Kill() is a blocking call, it waits for the thread to finish.
		// Hopefully that doesn't take too long
		Thread->Kill();
		delete Thread;
	}
}


#pragma endregion
// The code below will run on the new thread.


bool FAsyncConnectWorker::Init()
{
	UE_LOG(LogTemp, Warning, TEXT("My custom thread has been initialized"))

		// Return false if you want to abort the thread
		return true;
}


uint32 FAsyncConnectWorker::Run()
{
	// Peform your processor intensive task here. In this example, a neverending
	// task is created, which will only end when Stop is called.
    UE_LOG(LogTemp, Warning, TEXT("AsyncConnectWorker Run . . . . 1"));
    UE_LOG(LogTemp, Warning, TEXT("AsyncConnectWorker Run . . . . 2"));
    UE_LOG(LogTemp, Warning, TEXT("AsyncConnectWorker Run . . . . 3"));
    GameServerIP = "127.0.0.1";
    GameServerPort = 9000;
    CreateSocket();
    Start();
    // while (bRunThread)
	// {
	// 	UE_LOG(LogTemp, Warning, TEXT("My custom thread is running!"))
	// 		FPlatformProcess::Sleep(1.0f);
	// }

	return 0;
}


// This function is NOT run on the new thread!
void FAsyncConnectWorker::Stop()
{
	// Clean up memory usage here, and make sure the Run() function stops soon
	// The main thread will be stopped until this finishes!

	// For this example, we just need to terminate the while loop
	// It will finish in <= 1 sec, due to the Sleep()
	bRunThread = false;
}

bool FAsyncConnectWorker::SendMessageToServer(const FString& Message)
{
    // 메세지 전송
    int32 BytesSent = 0;

    return Socket->Send((uint8*)TCHAR_TO_UTF8(*Message), Message.Len(), BytesSent);
}

void FAsyncConnectWorker::CreateSocket()
{
    bool bIsValid = true;

    if (bIsValid)
    {
        // TCP 소켓 생성
//          Socket = SocketSubsystemPtr->CreateSocket(NAME_Stream, TEXT("YourSocketName"), false);

        Socket = ISocketSubsystem::Get(PLATFORM_SOCKETSUBSYSTEM)->CreateSocket(TEXT("Stream"), TEXT("Client Socket"));

        // 서버에 연결
        TSharedRef<FInternetAddr> ServerAddress = ISocketSubsystem::Get(PLATFORM_SOCKETSUBSYSTEM)->CreateInternetAddr();;

        FIPv4Address ip;
        FIPv4Address::Parse(GameServerIP, ip);

        ServerAddress->SetIp(ip.Value);
        ServerAddress->SetPort(GameServerPort);
        if (Socket->Connect(*ServerAddress))
        {
            // 메세지 전송
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

void FAsyncConnectWorker::Start()
{
    // CreateSocket();

    // Listen . . . 

    UE_LOG(LogTemp, Warning, TEXT("Connect Start . . . ."));
    isRun = true;
    while (isRun)
    {
        FPlatformProcess::Sleep(1.0f);
        // UE_LOG(LogTemp, Warning, TEXT("Listen . . . ."));

        uint32 pendingDataSize = 0;
        TArray<uint8> recvedData;

        int32 totalReadData = 0;

        Socket->SetNonBlocking(true);
        int32 _read;
        uint8 _temp;
        if (!Socket->Recv(&_temp, 1, _read, ESocketReceiveFlags::Peek))
        {
            UE_LOG(LogTemp, Warning, TEXT("Recv Failed"));
            isRun = false;
            continue;
        }

        Socket->SetNonBlocking(true);

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
                UE_LOG(LogTemp, Warning, TEXT("Data Recve Fail"));
                // Data Read Failed.
                break;
            }
            totalReadData = totalReadData + readData;
        }

        if (isRun && recvedData.Num() != 0)
        {
            UE_LOG(LogTemp, Warning, TEXT("Recved Data . . . ."));
            // Recv Logic
            RecvMessageFromServer(recvedData);
        }

        // sleep for loop control . . .
    }

    // DisConnect
}



void FAsyncConnectWorker::RecvMessageFromServer(TArray<uint8>& Message)
{
    // if (arenaGameMode)
    // {
    //     //        arenaGameMode->CallMessageFunctionByName();
    //     //        arenaGameMode->CallMessageFunctionByUnique();
    // }
    GEngine->AddOnScreenDebugMessage(-1, 10.f, FColor::Blue, TEXT("Data Received"));
    FString _data = ReadDataAsString(Message, Message.Num());

    PrintGameMessageAsDebugPrint( ConvertDataToGameMessage(_data) );
}

FString FAsyncConnectWorker::ReadDataAsString(TArray<uint8>& Message, int32 length)
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

FGameMessage FAsyncConnectWorker::ConvertDataToGameMessage(FString Message)
{
    FGameMessage result;

    return result;
}

void FAsyncConnectWorker::PrintGameMessageAsDebugPrint(FGameMessage msg)
{
    // GEngine->AddOnScreenDebugMessage
}

