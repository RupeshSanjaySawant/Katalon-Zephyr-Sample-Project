# Katalon and Zephyr Scale For Jira Cloud Integration
## Overview
This document will cover a step by step guide on integration of Zephyr Scale Cloud in JIRA with Katalon Automation Platform.

## Prerequisites
- Install Katalon Studio
- Install Zephyr Scale App in Jira Cloud

## Setup Zephyr Scale in Jira
- Generating API Access Tokens
You need to generate an access token to use the API for Zephyr Scale. The following steps illustrate the process. Each user interacting with the API will need a token for that instance of Jira.
  - Click your Jira profile icon and select Zephyr Scale API Access Tokens.
  - Click Create access token.
  - A pop-up window appears.
  - Click the copy button to copy the access token.

Now you're set to get started with using the REST API!
- The URL for API requests is:
https://api.zephyrscale.smartbear.com/v2/{endpoint}
The authorization we use is based on JWT, so you have to use a bearer token. Add the "Authorization" header with the value "Bearer {token}".

- API documentation
API documentation - review the docs for details on available endpoints and data models.

- Create Sample Test Case, Test Cycle, Test Plan and Link Them As per requirements

## Setup Katalon Project
- Global Variables Setup
global variable in your package which will be used in API calls for pushing execution results to Zephyr cloud in Jira from Katalon after execution of test cases.	

- Create Test Case and tag
Script the test case and give name to test case with prefix as Test Case Key from Zephyr Scale then underscore followed by Test case name  eg.
<<TestCase Key>>_<<Test case name as per your choice>>
Also, you need to tag the test case with linked Test Cycle Keys from Zephyr scale as below


- Web Service Request
Under the Object repository web service request for API Create test execution -This API  will be used to push test results by creating a new execution record for each Test case after execution in Katalon.
In API you need to provide end point, set authorization with bearer token generated in step Generating API Access Tokens , update that to HTTP Header and the HTTP Body with parameterized global variables which will be replaced with actual value in runtime.


- TestListener
test listener with following methods and code given as below FYR. This will call the Execution API after each Test case gets executed and push results to Zephyr Scale.

<< default import statements will be there>> 

class NewTestListener {

	/**
	 * Executes after every test case ends.
	 * @param testCaseContext related information of the executed test case.
	 */
	@AfterTestCase
	def sampleAfterTestCase(TestCaseContext testCaseContext) {
		
		//Get Test Case Key from test case name
		
		TestCase testCase = findTestCase(testCaseContext.getTestCaseId())
		GlobalVariable.g_ZS_TestCaseKey=testCase.getName().split("_")[0]
		
		
		//Set Zephyr Status
		if (testCaseContext.getTestCaseStatus()=="PASSED")
			GlobalVariable.g_ZS_StatusName="Pass"
		else if (testCaseContext.getTestCaseStatus()=="FAILED")
			GlobalVariable.g_ZS_StatusName="Fail"
		else
			GlobalVariable.g_ZS_StatusName="Not Executed"
		
		//Call API to Push Result to Zephyr by creating Test case execution		

		WS.sendRequest(findTestObject('Create Execution'))
		
	}

}




## Install Basic Search for Dynamic Test Suite Plugin
You can install Basic Search For Dynamic Test Suite by login into the katalon store and reload the plugin in Katalon Studio.


## Create Dynamic Test Suite and Generate Command for Console Mode
To create a new dynamic test suite, do as follows:
In the Tests Explorer panel, right-click at the Test Suites folder > New > Dynamic Test Suite.
- Create a new dynamic test suite

- Fill in the name and the description (optional).
We name it Dynamic Test Suite 1.
Create a new dynamic test suite dialog

- Click OK.

- Click generate Command icon near Run icon and select Test Suite and other details

## Execute Dynamic Test Suite Using Console Mode Command 
This command can be configured in Katalon TestOps or In CI Tool for triggering execution or can be run on cmd using Katalon Runtime Engine (KRE).

./katalonc -noSplash -runMode=console -projectPath="/Users/rupeshsawant/Katalon Studio/Katalon and Zephyr Integration/Katalon and Zephyr Integration.prj" -retry=0 -testSuitePath="Test Suites/Zephyr TestCycle Dynamic Test Suite" -browserType="Chrome" -executionProfile="default" -apiKey="<<Your APi Key>>" -orgID=<<Your Org Id>> --config -proxy.auth.option=NO_PROXY -proxy.system.option=NO_PROXY -proxy.system.applyToDesiredCapabilities=true -webui.autoUpdateDrivers=true


- You need to append 2 arguments with the Test Cycle Key value which you want to execute. 

  -testSuiteQuery="tag=(<<Your Test Cycle Key)>>)"
  This argument will be used for filtering test cases linked to test cycle for execution in dynamic testsuite and 

  -g_ZS_TestCycleKey="<<Your Test Cycle Key)>>"
  This argument will be used for set global variable value to selected  test cycle which will be required in API call

Updated command as follows :-

./katalonc -noSplash -runMode=console -projectPath="/Users/rupeshsawant/Katalon Studio/Katalon and Zephyr Integration/Katalon and Zephyr Integration.prj" -retry=0 -testSuitePath="Test Suites/Zephyr TestCycle Dynamic Test Suite" -browserType="Chrome" -executionProfile="default" -apiKey="<<Your APi Key>>" -orgID=<<Your Org Id>> --config -proxy.auth.option=NO_PROXY -proxy.system.option=NO_PROXY -proxy.system.applyToDesiredCapabilities=true -webui.autoUpdateDrivers=true -testSuiteQuery="tag=(<<Your Test Cycle Key)>>)" -g_ZS_TestCycleKey="<<Your Test Cycle Key)>>"


## Check Test results in Jira Zephyr Scale
Results will be updated in Jira Zephyr scale in respective test cycle and test case with new execution record.

