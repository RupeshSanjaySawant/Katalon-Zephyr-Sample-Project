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

WebUI.openBrowser('')

WebUI.navigateToUrl('https://arinslimindia.com/')

WebUI.click(findTestObject('Object Repository/Page_Arins SLiM INDiA/a_Book Online'))

WebUI.click(findTestObject('Object Repository/Page_Arins SLiM INDiA/button_BOOK'))

WebUI.click(findTestObject('Object Repository/Page_Arins SLiM INDiA/div_30'))

WebUI.click(findTestObject('Object Repository/Page_Arins SLiM INDiA/p_900 AM'))

WebUI.click(findTestObject('Object Repository/Page_Arins SLiM INDiA/button_Book (1)'))

WebUI.setText(findTestObject('Object Repository/Page_Arins SLiM INDiA/input__customer-first-name'), 'Rupesh')

WebUI.setText(findTestObject('Object Repository/Page_Arins SLiM INDiA/input__customer-last-name'), 'Sawant')

WebUI.setText(findTestObject('Object Repository/Page_Arins SLiM INDiA/input__email'), 'Rupesh.sawant@katalon.com')

WebUI.click(findTestObject('Object Repository/Page_Arins SLiM INDiA/label_Store my information for faster check_754145'))

WebUI.click(findTestObject('Object Repository/Page_Arins SLiM INDiA/button_Book More Services'))

WebUI.takeFullPageScreenshotAsCheckpoint('Confirmation ')

WebUI.closeBrowser()

