<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>1.3. Send Proxy Alias to get Proxy list - Failed - Data not found (Negative)</name>
   <tag></tag>
   <elementGuidId>96631497-5fe8-455f-b0f2-7888eddb2d58</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <smartLocatorEnabled>false</smartLocatorEnabled>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>false</autoUpdateContent>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n    \&quot;proxyListRequest\&quot;: {\n        \&quot;transactionCode\&quot;: \&quot;${transactionCode}\&quot;,\n        \&quot;transactionId\&quot;: \&quot;${transactionId}\&quot;,\n        \&quot;cifNumber\&quot;: \&quot;${cifNumber}\&quot;,\n        \&quot;proxyAlias\&quot;: \&quot;${proxyAlias}\&quot;,\n        \&quot;accountNumber\&quot;: \&quot;${accountNumber}\&quot;,\n        \&quot;cid\&quot;: \&quot;${cid}\&quot;,\n        \&quot;channelType\&quot;: \&quot;${channelType}\&quot;\n    }\n}&quot;,
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
      <webElementGuid>12a5bb39-e0d1-45f7-b908-a4e34f1d8d85</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>10.1.1</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <path></path>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${url}/KomiBifastOriginProxy.interfaces:proxyList</restUrl>
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
      <id>04b005d1-8de3-4442-84ba-b2f2f35852bb</id>
      <masked>false</masked>
      <name>Authorization</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>79178c2e-b1d2-4f43-88ac-a3f0b9358cb6</id>
      <masked>false</masked>
      <name>Content_Type</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>37fe348d-5c5e-4408-bd38-a83f6691a2b4</id>
      <masked>false</masked>
      <name>transactionCode</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>63b7792d-2ace-4a16-8411-3b7dbded0a89</id>
      <masked>false</masked>
      <name>transactionId</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>371e9356-8347-4fbc-b942-09c86e9658b2</id>
      <masked>false</masked>
      <name>cifNumber</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>7fd7b876-11a1-4984-9fc7-3a97c86ab75a</id>
      <masked>false</masked>
      <name>proxyAlias</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>d5e3f55f-a10d-4cf7-a218-cc84c34a3044</id>
      <masked>false</masked>
      <name>accountNumber</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>75831da8-57ca-469d-8877-485e1b91a055</id>
      <masked>false</masked>
      <name>cid</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>8f70520f-9e99-404d-8308-2870ea8a0f40</id>
      <masked>false</masked>
      <name>channelType</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.url</defaultValue>
      <description></description>
      <id>429b5363-c598-43b8-a4dc-64372ae063b3</id>
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
