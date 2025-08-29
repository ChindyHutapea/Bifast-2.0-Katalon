<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>5.3. ProxyAlias Inquiry by Secondary ID-Timeout</name>
   <tag></tag>
   <elementGuidId>ad747c23-d00b-4941-8409-6c7f1806b3b9</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <smartLocatorEnabled>false</smartLocatorEnabled>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>false</autoUpdateContent>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n    \&quot;proxyRegInquiryRequest\&quot;: {\n        \&quot;transactionId\&quot;: \&quot;${transactionId}\&quot;,\n        \&quot;transactionCode\&quot;: \&quot;${transactionCode}\&quot;,\n        \&quot;channelType\&quot;: \&quot;${cifNumber}\&quot;,\n        \&quot;customerSecondaryType\&quot;: \&quot;${customerSecondaryType}\&quot;,\n      \t\&quot;registrationId\&quot;: \&quot;${registrationId}\&quot;,\n      \t\&quot;customerAccountNumber\&quot;: \&quot;${customerAccountNumber}\&quot;,\n        \&quot;customerSecondaryValue\&quot;: \&quot;${customerSecondaryValue}\&quot;,\n        \&quot;cid\&quot;: \&quot;${cid}\&quot;\n        \n    }\n}&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>${Content_Type}</value>
      <webElementGuid>991fec3b-bfa6-4517-ace2-60019ce9dfe8</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Basic QWRtaW5pc3RyYXRvcjptYW5hZ2U=</value>
      <webElementGuid>685b2b50-aed3-45db-914f-c4978e56670e</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>10.1.1</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <path></path>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${url}/KomiBifastOriginProxy.interfaces:proxyRegInqBySecondaryId</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>9a396a94-a406-43ad-ad93-7282345811a6</id>
      <masked>false</masked>
      <name>Authorization</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>cde7127b-a863-4d87-932b-6ececb1843b7</id>
      <masked>false</masked>
      <name>Content_Type</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>7f58420a-9bc0-4b25-ba22-c57dc105faf5</id>
      <masked>false</masked>
      <name>transactionId</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>3a24d169-cec5-4463-9fb7-36c6a8adede6</id>
      <masked>false</masked>
      <name>transactionCode</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>20325a70-a76c-4525-84eb-fbdfa71eb618</id>
      <masked>false</masked>
      <name>channelType</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>f78d9eaf-63bb-4147-97b1-b470879e6c19</id>
      <masked>false</masked>
      <name>customerSecondaryType</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>5babdce3-fac8-457c-bf82-24bc0f14c977</id>
      <masked>false</masked>
      <name>customerSecondaryValue</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>612f21e5-0c8c-4af2-9c71-abf4ba8a5e25</id>
      <masked>false</masked>
      <name>cid</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.url</defaultValue>
      <description></description>
      <id>ec28b744-2985-4389-9e49-9d19292c8975</id>
      <masked>false</masked>
      <name>url</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>947999ae-aea1-4e83-b9d8-df72a467a7fd</id>
      <masked>false</masked>
      <name>registrationId</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>afa0f70f-1bd5-4e7a-aedb-17c87b03f703</id>
      <masked>false</masked>
      <name>customerAccountNumber</name>
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
