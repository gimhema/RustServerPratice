// Fill out your copyright notice in the Description page of Project Settings.

#pragma once

#include "Engine.h"
#include "GameFramework/Actor.h"
#include "Sockets.h"
#include "SocketSubsystem.h"
#include "../GameCommon/ArenaGameMode.h"
// #include "SocketSubsystemModule.h"
#include "IPAddress.h"
#include "ArenaGameMessage.h"
#include "ArenaServerConnector.generated.h"


UCLASS()
class UNREALARENACLIENT_API AArenaServerConnector : public AActor
{
	GENERATED_BODY()
	
public:	
	// Sets default values for this actor's properties
	AArenaServerConnector();

protected:
	// Called when the game starts or when spawned
	virtual void BeginPlay() override;

public:	
	// Called every frame
	virtual void Tick(float DeltaTime) override;

public:
	UPROPERTY()
	AArenaGameMode* arenaGameMode;

	UPROPERTY(BlueprintReadWrite)
	FString GameServerIP = TEXT("");
	UPROPERTY(BlueprintReadWrite)
	int32 GameServerPort = 0;

	
	ISocketSubsystem* SocketSubsystemPtr;
	FSocket* Socket;

	bool isRun = false;

public:
	// Initialize
	UFUNCTION(BlueprintCallable)
	void CreateSocket();

	UFUNCTION(BlueprintCallable)
	void InitArenaGameMode();

public:
	// Connect Loop
	UFUNCTION(BlueprintCallable)
	void Start();

public:
	// Send & Recv
	UFUNCTION(BlueprintCallable)
	bool SendMessageToServer(const FString& Message);

	UFUNCTION(BlueprintCallable)
	void RecvMessageFromServer(TArray<uint8>& Message);

	UFUNCTION(BlueprintCallable)
	void DisConnect();

public:
	// Data Method
	UFUNCTION()
	FString ReadDataAsString(TArray<uint8>& Message, int32 length);

	UFUNCTION()
	FArenaGameMessage ConvertDataToGameMessage(FString data);

};
