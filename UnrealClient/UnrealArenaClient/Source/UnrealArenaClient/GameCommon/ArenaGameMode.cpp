// Fill out your copyright notice in the Description page of Project Settings.


#include "ArenaGameMode.h"

void AArenaGameMode::InitializeFunctionMap()
{
	MessageFuncMap.Add(EArenaGameMessage::ECONSOLE_WRITE_LINE, CreateMessageFunc("RECV_ECONSOLE_WRITE_LINE_FUNC"));
}

void AArenaGameMode::CallMessageFunctionByUnique(EArenaGameMessage fUnique, FString fParam)
{
	if (MessageFuncMap[fUnique].IsBound())
	{
		MessageFuncMap[fUnique].Execute( ConvertDataToFuncParam(fParam) );
	}
}


FGameMessageFuncDelegate AArenaGameMode::CreateMessageFunc(FString funcName)
{
	FGameMessageFuncDelegate _delegate;
	_delegate.BindUFunction(this, FName(funcName));
	return _delegate;
}

TArray<float> AArenaGameMode::ConvertDataToFuncParam(FString splitedData)
{
	TArray<float> _arr;

	TArray<FString> Substrings;
	splitedData.ParseIntoArray(Substrings, TEXT(","), true);

	for (int i = 0; i < Substrings.Num(); ++i)
	{
		_arr.Push(FCString::Atof(*Substrings[i]));
	}

	return _arr;
}

void AArenaGameMode::RECV_ECONSOLE_WRITE_LINE_FUNC(TArray<float> param)
{

}


