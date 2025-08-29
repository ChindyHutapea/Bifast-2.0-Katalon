<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>PROXY MANAGEMENT OTP</name>
   <tag></tag>
   <elementGuidId>2ce64166-6a5d-4b89-aa86-b0be97ddf7eb</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <smartLocatorEnabled>false</smartLocatorEnabled>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>false</autoUpdateContent>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n    \&quot;jsonRequest\&quot;: {\n        \&quot;transactionId\&quot;: \&quot;${transactionId}\&quot;,\n        \&quot;transactionCode\&quot;: \&quot;${transactionCode}\&quot;,\n        \&quot;customerAccountType\&quot;: \&quot;${customerAccountType}\&quot;,\n        \&quot;customerAccountNumber\&quot;: \&quot;${customerAccountNumber}\&quot;,\n        \&quot;customerAccountName\&quot;: \&quot;${customerAccountName}\&quot;,\n        \&quot;customerResidentStatus\&quot;: \&quot;${customerResidentStatus}\&quot;,\n        \&quot;amountCurrency\&quot;: \&quot;${amountCurrency}\&quot;,\n        \&quot;customerCityName\&quot;: \&quot;${customerCityName}\&quot;,\n        \&quot;proxyType\&quot;: \&quot;${proxyType}\&quot;,\n        \&quot;proxyAlias\&quot;: \&quot;${proxyAlias}\&quot;,\n        \&quot;proxyOperationType\&quot;: \&quot;${proxyOperationType}\&quot;,\n        \&quot;cid\&quot;: \&quot;${cid}\&quot;,\n        \&quot;customerSecondaryValue\&quot;: \&quot;${customerSecondaryValue}\&quot;,\n        \&quot;channelType\&quot;: \&quot;${channelType}\&quot;,\n    }\n}&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>${Authorization}</value>
      <webElementGuid>97b4e646-3022-479b-b0b9-313d0d52be6d</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>${Content_Type}</value>
      <webElementGuid>991fec3b-bfa6-4517-ace2-60019ce9dfe8</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>10.1.1</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <path></path>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>http://184.169.41.135:5557/restv2/KomiBifastProxy.ws:proxy/proxyList</restUrl>
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
      <id>63b7792d-2ace-4a16-8411-3b7dbded0a89</id>
      <masked>false</masked>
      <name>transactionId</name>
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
      <id>371e9356-8347-4fbc-b942-09c86e9658b2</id>
      <masked>false</masked>
      <name>customerAccountType</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>7fd7b876-11a1-4984-9fc7-3a97c86ab75a</id>
      <masked>false</masked>
      <name>customerAccountNumber</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>14afc234-3f15-4a60-9ef1-e8cedbf50c87</id>
      <masked>false</masked>
      <name>customerAccountName</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>75831da8-57ca-469d-8877-485e1b91a055</id>
      <masked>false</masked>
      <name>customerResidentStatus</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>8f70520f-9e99-404d-8308-2870ea8a0f40</id>
      <masked>false</masked>
      <name>amountCurrency</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>4febee62-a73e-49c9-a41f-019e95f4f99d</id>
      <masked>false</masked>
      <name>customerCityName</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>c59e2693-db86-42f0-8d2d-4a78c91054bd</id>
      <masked>false</masked>
      <name>proxyType</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>7e5b0b86-c4e3-4746-b15d-01afca36c748</id>
      <masked>false</masked>
      <name>proxyAlias</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>ecbf84da-e054-48bf-8ea1-238cd356f5a0</id>
      <masked>false</masked>
      <name>proxyOperationType</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>2a977e0f-52dd-4c70-a51e-6d24f9f4bbb3</id>
      <masked>false</masked>
      <name>cid</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>d1fd4540-7f02-4ec2-be3b-c9695513b78e</id>
      <masked>false</masked>
      <name>customerSecondaryValue</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>977fef07-094a-4a93-895c-9bbf7466c93a</id>
      <masked>false</masked>
      <name>channelType</name>
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
