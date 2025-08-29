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

def variable = [:]

variable.put('Authorization', Authorization)

variable.put('ContentType', ContentType)

variable.put('requestID', requestID)

variable.put('requestDate', requestDate)

variable.put('numberOfTrx', numberOfTrx)

variable.put('settlementMethod', settlementMethod)

variable.put('endToEndID', endToEndID)

variable.put('transactionID', transactionID)

variable.put('channelType', channelType)

variable.put('transactionPurpose', transactionPurpose)

variable.put('settlementAmount', settlementAmount)

variable.put('settlementCurrency', settlementCurrency)

variable.put('settlementDate', settlementDate)

variable.put('chargeBearer', chargeBearer)

variable.put('originatingBIC', originatingBIC)

variable.put('receivingBIC', receivingBIC)

variable.put('paymentInfo', paymentInfo)

variable.put('relatedEndToEndID', relatedEndToEndID)

variable.put('name', name)

variable.put('natID', natID)

variable.put('accountID', accountID)

variable.put('accountType', accountType)

variable.put('type', type)

variable.put('rsdntSts', rsdntSts)

variable.put('twnNm', twnNm)

variable.put('namecreditor', namecreditor)

variable.put('natIDcreditor', natIDcreditor)

variable.put('accountIDcreditor', accountIDcreditor)

variable.put('accountTypecreditor', accountTypecreditor)

variable.put('proxyTypecreditor', proxyTypecreditor)

variable.put('proxyValuecreditor', proxyValuecreditor)

variable.put('typecreditor', typecreditor)

variable.put('rsdntStscreditor', rsdntStscreditor)

variable.put('twnNmcreditor', twnNmcreditor)

RequestObject request = findTestObject('null', variable)

def response = WS.sendRequest(request)

def bodyResponse = response.getResponseBodyContent()

//HttpTextBodyContent body = request.getBodyContent();
//println(body.getText());

WS.comment(bodyResponse)

JsonParser.prettier(bodyResponse)

CustomKeywords.'mii.validation4.compareResponseMessage'(bodyResponse, 'Successful')

