// Fill out your copyright notice in the Description page of Project Settings.


#include "ArenaGameMode.h"

void AArenaGameMode::InitializeFunctionMap()
{
	MessageFuncMap.Add(EArenaGameMessage::ECONSOLE_WRITE_LINE, CreateMessageFunc("RECV_ECONSOLE_WRITE_LINE_FUNC"));
}

void AArenaGameMode::CallMessageFunctionByUnique(EArenaGameMessage fUnique)
{
	if (MessageFuncMap[fUnique].IsBound())
	{
		MessageFuncMap[fUnique].Execute();
	}
}


FGameMessageFuncDelegate AArenaGameMode::CreateMessageFunc(FString funcName)
{
	FGameMessageFuncDelegate _delegate;
	_delegate.BindUFunction(this, FName(funcName));
	return _delegate;
}

void AArenaGameMode::RECV_ECONSOLE_WRITE_LINE_FUNC()
{

}


