#pragma once

#include "ArenaGameMessage.generated.h"

UENUM(BlueprintType)
enum class EArenaGameMessage : uint8
{
	ECONSOLE_WRITE_LINE UMETA(DisplayName = "Console Write Line"),
	ETEST_MESSAGE2 UMETA(DisplayName = "TEST2"),
	ETEST_MESSAGE3 UMETA(DisplayName = "TEST3"),
	ENONE UMETA(DisplayName = "None"),
	EERROR UMETA(DisplayName = "Error")
};

// sendMsg = "_command:_param"
USTRUCT(BlueprintType)
struct FArenaGameMessage
{
	GENERATED_BODY()

	UPROPERTY()
	int senderID;

	UPROPERTY()
	int functionHeader;

	UPROPERTY()
	FString fuctionParam;
};