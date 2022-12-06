<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>GetLinks</name>
   <tag></tag>
   <elementGuidId>6563279d-9725-4613-b742-a7701c9bc13b</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <authorizationRequest>
      <authorizationInfo>
         <entry>
            <key>bearerToken</key>
            <value>eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiJjYWY0Mzc3Ny04MzVmLTM4NzQtOWQ2YS02YTliN2MwMGJjYjMiLCJjb250ZXh0Ijp7ImJhc2VVcmwiOiJodHRwczpcL1wvcnVwZXNoLWthdGFsb24uYXRsYXNzaWFuLm5ldCIsInVzZXIiOnsiYWNjb3VudElkIjoiNjIxZTI5ZmNhMTI0NTAwMDY4ODZiY2RmIn19LCJpc3MiOiJjb20ua2Fub2FoLnRlc3QtbWFuYWdlciIsImV4cCI6MTY5MzYzNDg2OSwiaWF0IjoxNjYyMDk4ODY5fQ.KM6zWDQ0NonS7NNrtzRyfMPeh50n1TyOEJxhE_</value>
         </entry>
      </authorizationInfo>
      <authorizationType>Bearer</authorizationType>
   </authorizationRequest>
   <connectionTimeout>0</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiJjYWY0Mzc3Ny04MzVmLTM4NzQtOWQ2YS02YTliN2MwMGJjYjMiLCJjb250ZXh0Ijp7ImJhc2VVcmwiOiJodHRwczpcL1wvcnVwZXNoLWthdGFsb24uYXRsYXNzaWFuLm5ldCIsInVzZXIiOnsiYWNjb3VudElkIjoiNjIxZTI5ZmNhMTI0NTAwMDY4ODZiY2RmIn19LCJpc3MiOiJjb20ua2Fub2FoLnRlc3QtbWFuYWdlciIsImV4cCI6MTY5MzYzNDg2OSwiaWF0IjoxNjYyMDk4ODY5fQ.KM6zWDQ0NonS7NNrtzRyfMPeh50n1TyOEJxhE_</value>
      <webElementGuid>959a4602-0d40-40e6-b5d2-468280dfd94d</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.4.1</katalonVersion>
   <maxResponseSize>0</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>GET</restRequestMethod>
   <restUrl>https://api.zephyrscale.smartbear.com/v2/testexecutions?testCycle=${GlobalVariable.g_ZS_TestCycleKey}</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>0</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>'KZDP-R1'</defaultValue>
      <description></description>
      <id>91850d47-1b6b-4be9-bbc1-81ff6604284d</id>
      <masked>false</masked>
      <name>testCycleIdOrKey</name>
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
