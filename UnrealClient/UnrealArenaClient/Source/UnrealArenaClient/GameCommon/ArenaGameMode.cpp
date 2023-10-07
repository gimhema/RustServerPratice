// Fill out your copyright notice in the Description page of Project Settings.


#include "ArenaGameMode.h"

void AArenaGameMode::InitializeFunctionMap()
{
	MessageFuncMap.Add(EArenaGameMessage::ECONSOLE_WRITE_LINE, CreateMessageFunc("RECV_ECONSOLE_WRITE_LINE_FUNC"));
}

void AArenaGameMode::RECV_ECONSOLE_WRITE_LINE_FUNC()
{

}

FGameMessageFuncDelegate AArenaGameMode::CreateMessageFunc(FString funcName)
{
	FGameMessageFuncDelegate _delegate;
	_delegate.BindUFunction(this, FName(funcName));
	return _delegate;
}