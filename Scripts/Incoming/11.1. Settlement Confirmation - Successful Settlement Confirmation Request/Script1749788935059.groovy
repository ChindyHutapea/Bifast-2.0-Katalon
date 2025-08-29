import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testng.keyword.TestNGBuiltinKeywords as TestNGKW
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import internal.GlobalVariable as GlobalVariable
import org.openqa.selenium.Keys as Keys
import com.kms.katalon.core.testobject.RequestObject as RequestObject
import com.kms.katalon.core.util.KeywordUtil as KeywordUtil
import mii.JsonParser as JsonParser
import groovy.json.JsonSlurper

def variable = [:]

variable.put('Authorization', Authorization)

variable.put('ContentType', ContentType)

variable.put('requestID', requestID)

variable.put('requestDate', requestDate)

variable.put('originalRequestID', originalRequestID)

variable.put('msgNmID', msgNmID)

variable.put('originalRequestDate', originalRequestDate)

variable.put('originalEnd2EndID', originalEnd2EndID)

variable.put('originalTransactionID', originalTransactionID)

variable.put('statusCode', statusCode)

variable.put('reasonCode', reasonCode)

variable.put('reasonAdditionalInfo', reasonAdditionalInfo)

variable.put('settlementDate', settlementDate)

variable.put('name', name)

variable.put('natID', natID)

variable.put('accountID', accountID)

variable.put('accountType', accountType)

variable.put('agentID', agentID)

variable.put('type', type)

variable.put('rsdntSts', rsdntSts)

variable.put('twnNm', twnNm)

variable.put('settlementAccountID', settlementAccountID)

variable.put('agentIDcreditor', agentIDcreditor)

variable.put('namecreditor', namecreditor)

variable.put('accountIDcreditor', accountIDcreditor)

variable.put('accountTypecreditor', accountTypecreditor)

variable.put('typecreditor', typecreditor)

variable.put('natIDcreditor', natIDcreditor)

variable.put('rsdntStscreditor', rsdntStscreditor)

variable.put('twnNmcreditor', twnNmcreditor)

variable.put('settlementAccountIDcreditor', settlementAccountIDcreditor)

RequestObject request = findTestObject('null', variable)

def response = WS.sendRequest(request)

def bodyResponse = response.getResponseBodyContent()

WS.comment(bodyResponse)

JsonParser.prettier(bodyResponse)

//CustomKeywords.'mii.settlement.compareResponseMessage'(bodyResponse, '{}')

WS.verifyResponseStatusCode(response, 200)

//WS.verifyElementPropertyValue(response, '{}', 1)

