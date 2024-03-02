// Fill out your copyright notice in the Description page of Project Settings.

#pragma once

#include "CoreMinimal.h"
#include "Sockets.h"
#include "SocketSubsystem.h"
#include "IPAddress.h"
#include "HAL/Runnable.h"

/**
 * 
 */

struct FGameMessage
{
	int64 senderID;
	int64 targetID;
	int64 functionHeader;
	TArray<float> functionParam;
	FString functionStringParam;
};

class UNREALARENACLIENT_API FAsyncConnectWorker : public FRunnable
{
public:

	// Constructor, create the thread by calling this
	FAsyncConnectWorker();

	// Destructor
	virtual ~FAsyncConnectWorker() override;


	// Overriden from FRunnable
	// Do not call these functions youself, that will happen automatically
	bool Init() override; // Do your setup here, allocate memory, ect.
	uint32 Run() override; // Main data processing happens here
	void Stop() override; // Clean up any memory you allocated here


private:

	// Thread handle. Control the thread using this, with operators like Kill and Suspend
	FRunnableThread* Thread;

	// Used to know when the thread should exit, changed in Stop(), read in Run()
	bool bRunThread;
	
public:

	UPROPERTY(BlueprintReadWrite)
	FString GameServerIP = TEXT("");
	UPROPERTY(BlueprintReadWrite)
	int32 GameServerPort = 0;


	ISocketSubsystem* SocketSubsystemPtr;
	
	FSocket* Socket;

	bool isRun = false;

public:
	void Start();

	void CreateSocket();

	void RecvMessageFromServer(TArray<uint8>& Message);

	bool SendMessageToServer(const FString& Message);

	FString ReadDataAsString(TArray<uint8>& Message, int32 length);

	void DisConnect();

public:
	
	FGameMessage ConvertDataToGameMessage(FString Message);
	void PrintGameMessageAsDebugPrint(FGameMessage msg);
};
