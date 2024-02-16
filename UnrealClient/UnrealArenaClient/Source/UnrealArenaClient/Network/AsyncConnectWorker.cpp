// Fill out your copyright notice in the Description page of Project Settings.


#include "AsyncConnectWorker.h"


#pragma region Main Thread Code
// This code will be run on the thread that invoked this thread (i.e. game thread)


FAsyncConnectWorker::FAsyncConnectWorker(/* You can pass in inputs here */)
{
	// Constructs the actual thread object. It will begin execution immediately
	// If you've passed in any inputs, set them up before calling this.
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
	while (bRunThread)
	{
		UE_LOG(LogTemp, Warning, TEXT("My custom thread is running!"))
			FPlatformProcess::Sleep(1.0f);
	}

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


void FAsyncConnectWorker::Start()
{

}

void FAsyncConnectWorker::CreateSocket()
{

}

void FAsyncConnectWorker::RecvMessageFromServer(TArray<uint8>& Message)
{
	
}

