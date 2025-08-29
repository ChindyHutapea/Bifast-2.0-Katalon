<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>3.3. Send Alias Resolution - Failed - Service Error(500)</name>
   <tag></tag>
   <elementGuidId>3db6af47-f2f7-4e74-8a12-abf1638c550f</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <smartLocatorEnabled>false</smartLocatorEnabled>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>false</autoUpdateContent>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n    \&quot;aliasResolutionInput\&quot;: {\n        \&quot;proxyType\&quot;: \&quot;${proxyType}\&quot;,\n        \&quot;proxyAlias\&quot;: \&quot;${proxyAlias}\&quot;,\n        \&quot;endToEndId\&quot;: \&quot;${endToEndId}{{trxId}}\&quot;,\n        \&quot;senderAccountId\&quot;: \&quot;${senderAccountId}\&quot;,\n        \&quot;withSaf\&quot;: \&quot;${withSaf}\&quot;\n    },  \&quot;ctx\&quot;: {\n        \&quot;komiUniqueId\&quot;: \&quot;${komiUniqueId}\&quot;\n    }\n}&quot;,
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
      <webElementGuid>19962620-9fef-4b24-91bb-6e0bdef0ba62</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>10.1.1</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <path></path>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${url}/KomiBifastOriginProxy.ws:proxy/aliasResolution</restUrl>
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
      <id>3a24d169-cec5-4463-9fb7-36c6a8adede6</id>
      <masked>false</masked>
      <name>proxyType</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>f78d9eaf-63bb-4147-97b1-b470879e6c19</id>
      <masked>false</masked>
      <name>proxyAlias</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>def59953-06a0-45de-bfbe-9cb1b2b2ab4c</id>
      <masked>false</masked>
      <name>endToEndId</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>407c775e-0497-4b2d-ad95-d9c55c22dd6c</id>
      <masked>false</masked>
      <name>senderAccountId</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>5babdce3-fac8-457c-bf82-24bc0f14c977</id>
      <masked>false</masked>
      <name>withSaf</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>20325a70-a76c-4525-84eb-fbdfa71eb618</id>
      <masked>false</masked>
      <name>ctx</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>857ffc9f-277b-43ee-ae45-515e4489b045</id>
      <masked>false</masked>
      <name>komiUniqueId</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.url</defaultValue>
      <description></description>
      <id>d0ae7d2a-b918-4d93-8e83-52d20a55ffe5</id>
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
