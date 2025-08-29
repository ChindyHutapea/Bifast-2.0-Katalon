<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>6.4. Akun Enquiry - Failed - Account Not Found</name>
   <tag></tag>
   <elementGuidId>dd0c1032-cb22-4ba0-9535-0372d4d54dbd</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <smartLocatorEnabled>false</smartLocatorEnabled>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>false</autoUpdateContent>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n    \&quot;accountInquiryRequest\&quot;: {\n        \&quot;transactionId\&quot;: \&quot;${transactionId}\&quot;,\n        \&quot;transactionCode\&quot;: \&quot;${transactionCode}\&quot;,\n        \&quot;cid\&quot;: \&quot;${cid}\&quot;,\n        \&quot;channelType\&quot;: \&quot;${channelType}\&quot;,\n        \&quot;branchInput\&quot;: \&quot;${branchInput}\&quot;,\n        \&quot;debitedAccountNumber\&quot;: \&quot;${debitedAccountNumber}\&quot;,\n        \&quot;creditedBic\&quot;: \&quot;${creditedBic}\&quot;,\n        \&quot;creditedProxyType\&quot;: \&quot;${creditedProxyType}\&quot;,\n        \&quot;creditedProxyAlias\&quot;: \&quot;${creditedProxyAlias}\&quot;,\n        \&quot;chargeType\&quot;: \&quot;${chargeType}\&quot;,\n        \&quot;trxAmount\&quot;: \&quot;${trxAmount}\&quot;,\n        \&quot;feeAmount\&quot;: \&quot;${feeAmount}\&quot;,\n        \&quot;amountCurrency\&quot;: \&quot;${amountCurrency}\&quot;,\n        \&quot;categoryPurpose\&quot;: \&quot;${categoryPurpose}\&quot;,\n        \&quot;chargeBearerCode\&quot;: \&quot;${chargeBearerCode}\&quot;,\n        \&quot;userInput\&quot;: \&quot;${userInput}\&quot;\n    }\n}&quot;,
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
      <webElementGuid>f454741b-2010-4e5a-8e85-17bf4f52f2d0</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>10.1.1</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <path></path>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${url}/KomiBifastOriginAE.interfaces:accountInquiry</restUrl>
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
      <id>37fe348d-5c5e-4408-bd38-a83f6691a2b4</id>
      <masked>false</masked>
      <name>transactionCode</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>371e9356-8347-4fbc-b942-09c86e9658b2</id>
      <masked>false</masked>
      <name>channelType</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>7fd7b876-11a1-4984-9fc7-3a97c86ab75a</id>
      <masked>false</masked>
      <name>cid</name>
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
      <id>14afc234-3f15-4a60-9ef1-e8cedbf50c87</id>
      <masked>false</masked>
      <name>branchInput</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>75831da8-57ca-469d-8877-485e1b91a055</id>
      <masked>false</masked>
      <name>debitedAccountNumber</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>8f70520f-9e99-404d-8308-2870ea8a0f40</id>
      <masked>false</masked>
      <name>creditedBic</name>
   </variables>
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
      <id>4febee62-a73e-49c9-a41f-019e95f4f99d</id>
      <masked>false</masked>
      <name>creditedProxyType</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>c59e2693-db86-42f0-8d2d-4a78c91054bd</id>
      <masked>false</masked>
      <name>creditedProxyAlias</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>7e5b0b86-c4e3-4746-b15d-01afca36c748</id>
      <masked>false</masked>
      <name>trxAmount</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>ecbf84da-e054-48bf-8ea1-238cd356f5a0</id>
      <masked>false</masked>
      <name>chargeType</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>2a977e0f-52dd-4c70-a51e-6d24f9f4bbb3</id>
      <masked>false</masked>
      <name>amountCurrency</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>d1fd4540-7f02-4ec2-be3b-c9695513b78e</id>
      <masked>false</masked>
      <name>feeAmount</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>977fef07-094a-4a93-895c-9bbf7466c93a</id>
      <masked>false</masked>
      <name>chargeBearerCode</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>810e0c7d-8edf-4373-8736-808a5741a6fe</id>
      <masked>false</masked>
      <name>categoryPurpose</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>ce28b43c-14af-4275-afb1-46267eb12fe6</id>
      <masked>false</masked>
      <name>userInput</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.url</defaultValue>
      <description></description>
      <id>83aff072-2802-4100-833a-0c756e2239e3</id>
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
