<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Create Execution</name>
   <tag></tag>
   <elementGuidId>ec83f2bb-1cba-4b18-839c-ec01669d8f3e</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <authorizationRequest>
      <authorizationInfo>
         <entry>
            <key>bearerToken</key>
            <value>eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiJjYWY0Mzc3Ny04MzVmLTM4NzQtOWQ2YS02YTliN2MwMGJjYjMiLCJjb250ZXh0Ijp7ImJhc2VVcmwiOiJodHRwczpcL1wvcnVwZXNoLWthdGFsb24uYXRsYXNzaWFuLm5ldCIsInVzZXIiOnsiYWNjb3VudElkIjoiNjIxZTI5ZmNhMTI0NTAwMDY4ODZiY2RmIn19LCJpc3MiOiJjb20ua2Fub2FoLnRlc3QtbWFuYWdlciIsImV4cCI6MTY5MzYzNDg2OSwiaWF0IjoxNjYyMDk4ODY5fQ.KM6zWDQ0NonS7NNrtzRyfMPeh50n1TyOEJxhE_sT3hU</value>
         </entry>
      </authorizationInfo>
      <authorizationType>Bearer</authorizationType>
   </authorizationRequest>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;projectKey\&quot;: \&quot;${GlobalVariable.g_ZS_ProjectKey}\&quot;,\n  \&quot;testCaseKey\&quot;: \&quot;${GlobalVariable.g_ZS_TestCaseKey}\&quot;,\n  \&quot;testCycleKey\&quot;: \&quot;${GlobalVariable.g_ZS_TestCycleKey}\&quot;,\n  \&quot;statusName\&quot;: \&quot;${GlobalVariable.g_ZS_StatusName}\&quot;\n}&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>48539942-3086-42bd-bbff-fa19720d0d40</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiJjYWY0Mzc3Ny04MzVmLTM4NzQtOWQ2YS02YTliN2MwMGJjYjMiLCJjb250ZXh0Ijp7ImJhc2VVcmwiOiJodHRwczpcL1wvcnVwZXNoLWthdGFsb24uYXRsYXNzaWFuLm5ldCIsInVzZXIiOnsiYWNjb3VudElkIjoiNjIxZTI5ZmNhMTI0NTAwMDY4ODZiY2RmIn19LCJpc3MiOiJjb20ua2Fub2FoLnRlc3QtbWFuYWdlciIsImV4cCI6MTY5MzYzNDg2OSwiaWF0IjoxNjYyMDk4ODY5fQ.KM6zWDQ0NonS7NNrtzRyfMPeh50n1TyOEJxhE_</value>
      <webElementGuid>64541faa-6da7-4d95-be62-246882c6b1c7</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.4.1</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>https://api.zephyrscale.smartbear.com/v2/testexecutions</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
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
