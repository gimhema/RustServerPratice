#pragma once

UENUM(BlueprintType)
enum class EArenaGameMessage : uint8
{
	ECONSOLE_WRITE_LINE UMETA(DisplayName = "Console Write Line"),
	ETEST_MESSAGE2 UMETA(DisplayName = "TEST2"),
	ETEST_MESSAGE3 UMETA(DisplayName = "TEST3"),
	ENONE UMETA(DisplayName = "None"),
	EERROR UMETA(DisplayName = "Error")
};