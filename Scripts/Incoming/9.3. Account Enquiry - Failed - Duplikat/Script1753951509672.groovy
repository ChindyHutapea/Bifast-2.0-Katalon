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

variable.put('Content_Type', Content_Type)

variable.put('AppHdrId', AppHdrId)

variable.put('ToId', ToId)

variable.put('BizMsgIdr', BizMsgIdr)

variable.put('MsgDefIdr', MsgDefIdr)

variable.put('CreDt', CreDt)

variable.put('GrpHdrCreDtTm', GrpHdrCreDtTm)

variable.put('GrpHdrMsgId', GrpHdrMsgId)

variable.put('GrpHdrNbOfTxs', GrpHdrNbOfTxs)

variable.put('SttlmInfSttlmMtd', SttlmInfSttlmMtd)

variable.put('PmtIdEndToEndId', PmtIdEndToEndId)

variable.put('PmtIdTxId', PmtIdTxId)

variable.put('PmtIdClrSysRef', PmtIdClrSysRef)

variable.put('CtgyPurpPrtry', CtgyPurpPrtry)

variable.put('IntrBkSttlmDt', IntrBkSttlmDt)

variable.put('ChrgBr', ChrgBr)

variable.put('FinInstnIdOthrId', FinInstnIdOthrId)

variable.put('OthrId', OthrId)

variable.put('CdtrAcctOthrId', CdtrAcctOthrId)

variable.put('Value', Value)

variable.put('Ccy', Ccy)

RequestObject request = findTestObject('Incoming/Postman/9.3. Account Enquiry - Failed - Duplikat (U149)', variable)

def response = WS.sendRequest(request)

def bodyResponse = response.getResponseBodyContent()

WS.comment(bodyResponse)

JsonParser.prettier(bodyResponse)

String actualResponse = response.getResponseText()

// Jika masih ada bracket, coba hapus secara manual
actualResponse = actualResponse.replace('[', '').replace(']', '')

println('Actual Response (cleaned): ' + actualResponse)

CustomKeywords.'mii.AccountInquiryIncoming.comparePrtry'(actualResponse, 'U149')

