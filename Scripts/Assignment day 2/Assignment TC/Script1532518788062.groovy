import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.checkpoint.CheckpointFactory as CheckpointFactory
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as MobileBuiltInKeywords
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testcase.TestCaseFactory as TestCaseFactory
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testdata.TestDataFactory as TestDataFactory
import com.kms.katalon.core.testobject.ObjectRepository as ObjectRepository
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WSBuiltInKeywords
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUiBuiltInKeywords
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import internal.GlobalVariable as GlobalVariable

WebUI.openBrowser('')

WebUI.navigateToUrl('http://www.seleniumeasy.com/test/')

WebUI.click(findTestObject('Assignment objects/Page_Selenium Easy - Best Demo webs/i_tree-indicator glyphicon gly'))

WebUI.click(findTestObject('Assignment objects/Page_Selenium Easy - Best Demo webs/a_Simple Form Demo'))

WebUI.setText(findTestObject('Assignment objects/Page_Selenium Easy Demo - Simple Fo/input_user-message'), 'Hi')

WebUI.click(findTestObject('Assignment objects/Page_Selenium Easy Demo - Simple Fo/button_Show Message'))

WebUI.verifyElementPresent(findTestObject('Assignment objects/Page_Selenium Easy Demo - Simple Fo/div_Your Message'), 10)

WebUI.setText(findTestObject('Assignment objects/Page_Selenium Easy Demo - Simple Fo/input_sum1'), '5')

WebUI.setText(findTestObject('Assign Objects/Page_Selenium Easy Demo - Simple Fo/input_sum2'), '5')

WebUI.click(findTestObject('Assignment objects/Page_Selenium Easy Demo - Simple Fo/button_Get Total'))

WebUI.verifyElementPresent(findTestObject('Assign Objects/Page_Selenium Easy Demo - Simple Fo/div_Total a  b'), 5)

