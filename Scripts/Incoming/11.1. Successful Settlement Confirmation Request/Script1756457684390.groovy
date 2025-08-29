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

variable.put('AppHdrId', AppHdrId)

variable.put('ToId', ToId)

variable.put('BizMsgIdr', BizMsgIdr)

variable.put('MsgDefIdr', MsgDefIdr)

variable.put('BizSvc', BizSvc)

variable.put('CreDt', CreDt)

variable.put('GrpHdrCreDtTm', GrpHdrCreDtTm)

variable.put('GrpHdrMsgId', GrpHdrMsgId)

variable.put('OrgnlMsgNmId', OrgnlMsgNmId)

variable.put('TxInfAndStsOrgnlEndToEndId', TxInfAndStsOrgnlEndToEndId)

variable.put('TxInfAndStsOrgnlTxId', TxInfAndStsOrgnlTxId)

variable.put('TxInfAndStsTxSts', TxInfAndStsTxSts)

variable.put('StsRsnInfPrtry', StsRsnInfPrtry)

variable.put('ClrSysRef', ClrSysRef)

variable.put('IntrBkSttlmDt', IntrBkSttlmDt)

variable.put('DbtrAcctId', DbtrAcctId)

variable.put('DbtrAgtId', DbtrAgtId)

variable.put('CdtrAgtId', CdtrAgtId)

variable.put('CdtrNm', CdtrNm)

variable.put('CdtrAcctOthrId', CdtrAcctOthrId)

variable.put('TpPrtry', TpPrtry)

variable.put('DbtrAgtAcctId', DbtrAgtAcctId)

variable.put('CdtrAgtAcctId', CdtrAgtAcctId)

RequestObject request = findTestObject('Incoming/Postman/Settlement Confirmation', variable)

def response = WS.sendRequest(request)

def bodyResponse = response.getResponseBodyContent()

WS.comment(bodyResponse)

JsonParser.prettier(bodyResponse)

//CustomKeywords.'mii.settlement.compareResponseMessage'(bodyResponse, '{}')

WS.verifyResponseStatusCode(response, 200)

//WS.verifyElementPropertyValue(response, '{}', 1)

