// Fill out your copyright notice in the Description page of Project Settings.

#pragma once

#include "Engine.h"
#include "GameFramework/Actor.h"
#include "Sockets.h"
#include "SocketSubsystem.h"
// #include "SocketSubsystemModule.h"
#include "IPAddress.h"
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
	UPROPERTY(BlueprintReadWrite)
	FString GameServerIP = TEXT("");
	UPROPERTY(BlueprintReadWrite)
	int32 GameServerPort = 0;

//	FSocketSubsystemModule& SocketSubsystem = FModuleManager::LoadModuleChecked<FSocketSubsystemModule>("SocketSubsystem");
	ISocketSubsystem* SocketSubsystemPtr;
	FSocket* Socket;

	bool isRun = false;

public:
	// Initialize
	UFUNCTION()
		void CreateSocket();

public:
	// Connect Loop
	UFUNCTION()
		void Start();

public:
	// Send & Recv
	UFUNCTION()
		bool SendMessageToServer(const FString& Message);

	UFUNCTION()
		void RecvMessageFromServer();

	UFUNCTION()
		void DisConnect();

};
