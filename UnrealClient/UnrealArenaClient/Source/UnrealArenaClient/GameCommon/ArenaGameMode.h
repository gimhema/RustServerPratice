// Fill out your copyright notice in the Description page of Project Settings.

#pragma once

#include "CoreMinimal.h"
#include "GameFramework/GameModeBase.h"
#include "../Network/ArenaGameMessage.h"
#include "ArenaGameMode.generated.h"

/**
 * 
 */

DECLARE_DELEGATE(FGameMessageFuncDelegate);

UCLASS()
class UNREALARENACLIENT_API AArenaGameMode : public AGameModeBase
{
	GENERATED_BODY()
public:

public:
	TMap<EArenaGameMessage, FGameMessageFuncDelegate> MessageFuncMap;

public:
	void InitializeFunctionMap();
	FGameMessageFuncDelegate CreateMessageFunc(FString funcName);

public:
	// Function Maps . . . .
	void RECV_ECONSOLE_WRITE_LINE_FUNC();

};