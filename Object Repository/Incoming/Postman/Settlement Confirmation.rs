<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Settlement Confirmation</name>
   <tag></tag>
   <elementGuidId>8d4c7cf8-8a0f-4584-95c9-f738169afcbe</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <smartLocatorEnabled>false</smartLocatorEnabled>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>true</autoUpdateContent>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n    \&quot;BusMsg\&quot;: {\n        \&quot;AppHdr\&quot;: {\n            \&quot;Fr\&quot;: {\n                \&quot;FIId\&quot;: {\n                    \&quot;FinInstnId\&quot;: {\n                        \&quot;Othr\&quot;: {\n                            \&quot;Id\&quot;: \&quot;FASTIDJA\&quot;\n                        }\n                    }\n                }\n            },\n            \&quot;To\&quot;: {\n                \&quot;FIId\&quot;: {\n                    \&quot;FinInstnId\&quot;: {\n                        \&quot;Othr\&quot;: {\n                            \&quot;Id\&quot;: \&quot;MEGAIDJA\&quot;\n                        }\n                    }\n                }\n            },\n            \&quot;BizMsgIdr\&quot;: \&quot;20250618PDJBIDJA010O25008577\&quot;,\n            \&quot;MsgDefIdr\&quot;: \&quot;pacs.002.001.10\&quot;,\n            \&quot;BizSvc\&quot;: \&quot;STTL\&quot;,\n            \&quot;CreDt\&quot;: \&quot;2025-02-11T01:59:40Z\&quot;\n        },\n        \&quot;Document\&quot;: {\n            \&quot;FIToFIPmtStsRpt\&quot;: {\n                \&quot;GrpHdr\&quot;: {\n                    \&quot;MsgId\&quot;: \&quot;20250211PDJBIDJA01053898313\&quot;,\n                    \&quot;CreDtTm\&quot;: \&quot;2025-02-11T08:59:32.432\&quot;\n                },\n                \&quot;OrgnlGrpInfAndSts\&quot;: [\n                    {\n                        \&quot;OrgnlMsgId\&quot;: \&quot;20250508PDJBIDJA010O25003605\&quot;,\n                        \&quot;OrgnlMsgNmId\&quot;: \&quot;pacs.008.001.08\&quot;\n                    }\n                ],\n                \&quot;TxInfAndSts\&quot;: [\n                    {\n                        \&quot;OrgnlEndToEndId\&quot;: \&quot;20250508PDJBIDJA010O25003602\&quot;,\n                        \&quot;OrgnlTxId\&quot;: \&quot;20250211BMRIIDJA01002104320\&quot;,\n                        \&quot;TxSts\&quot;: \&quot;ACSC\&quot;,\n                        \&quot;StsRsnInf\&quot;: [\n                            {\n                                \&quot;Rsn\&quot;: {\n                                    \&quot;Prtry\&quot;: \&quot;U000\&quot;\n                                }\n                            }\n                        ],\n                        \&quot;ClrSysRef\&quot;: \&quot;002\&quot;,\n                        \&quot;OrgnlTxRef\&quot;: {\n                            \&quot;IntrBkSttlmDt\&quot;: \&quot;2025-02-11\&quot;,\n                            \&quot;Dbtr\&quot;: {\n                                \&quot;Pty\&quot;: {\n                                    \&quot;Nm\&quot;: \&quot;AC007911W15E100\&quot;\n                                }\n                            },\n                            \&quot;DbtrAcct\&quot;: {\n                                \&quot;Id\&quot;: {\n                                    \&quot;Othr\&quot;: {\n                                        \&quot;Id\&quot;: \&quot;0021871109100\&quot;\n                                    }\n                                }\n                            },\n                            \&quot;DbtrAgt\&quot;: {\n                                \&quot;FinInstnId\&quot;: {\n                                    \&quot;Othr\&quot;: {\n                                        \&quot;Id\&quot;: \&quot;BMRIIDJA\&quot;\n                                    }\n                                }\n                            },\n                            \&quot;CdtrAgt\&quot;: {\n                                \&quot;FinInstnId\&quot;: {\n                                    \&quot;Othr\&quot;: {\n                                        \&quot;Id\&quot;: \&quot;BMRIIDJA\&quot;\n                                    }\n                                }\n                            },\n                            \&quot;Cdtr\&quot;: {\n                                \&quot;Pty\&quot;: {\n                                    \&quot;Nm\&quot;: \&quot;KU000022000462140\&quot;\n                                }\n                            },\n                            \&quot;CdtrAcct\&quot;: {\n                                \&quot;Id\&quot;: {\n                                    \&quot;Othr\&quot;: {\n                                        \&quot;Id\&quot;: \&quot;0019845621100\&quot;\n                                    }\n                                },\n                                \&quot;Tp\&quot;: {\n                                    \&quot;Prtry\&quot;: \&quot;SVGS\&quot;\n                                }\n                            }\n                        },\n                        \&quot;SplmtryData\&quot;: [\n                            {\n                                \&quot;Envlp\&quot;: {\n                                    \&quot;Dtl\&quot;: {\n                                        \&quot;DbtrAgtAcct\&quot;: {\n                                            \&quot;Id\&quot;: {\n                                                \&quot;Othr\&quot;: {\n                                                    \&quot;Id\&quot;: \&quot;524110000980\&quot;\n                                                }\n                                            }\n                                        },\n                                        \&quot;CdtrAgtAcct\&quot;: {\n                                            \&quot;Id\&quot;: {\n                                                \&quot;Othr\&quot;: {\n                                                    \&quot;Id\&quot;: \&quot;0019845621100\&quot;\n                                                }\n                                            }\n                                        }\n                                    }\n                                }\n                            }\n                        ]\n                    }\n                ]\n            }\n        }\n    }\n}&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Basic QWRtaW5pc3RyYXRvcjptYW5hZ2U=</value>
      <webElementGuid>ed9b4be6-34b3-430e-9c8d-c85b1d5979f3</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>6609bcbe-de0d-4530-872a-f73d2c502758</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>10.2.3</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <path></path>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${url}/settlement-credit-transfer</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.url</defaultValue>
      <description></description>
      <id>e6864599-7113-4a9e-85a7-759a9f568c06</id>
      <masked>false</masked>
      <name>url</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
