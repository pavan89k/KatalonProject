<?xml version="1.0" encoding="UTF-8"?>
<WebElementEntity>
   <description></description>
   <name>html_Selenium Easy Demo - Simp</name>
   <tag></tag>
   <elementGuidId>64f036f9-d681-4a7c-824f-640afde94937</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>tag</name>
      <type>Main</type>
      <value>html</value>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>lang</name>
      <type>Main</type>
      <value>en</value>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>text</name>
      <type>Main</type>
      <value>
    
    
    
    
    
    
    
    Selenium Easy Demo - Simple Form to Automate using Selenium
    
    
    
    
    
    
    

#katalon{font-family:monospace;font-size:13px;background-color:rgba(0,0,0,.7);position:fixed;top:0;left:0;right:0;display:block;z-index:999999999;line-height: normal} #katalon div{padding:0;margin:0;color:#fff;} #katalon kbd{display:inline-block;padding:3px 5px;font:13px Consolas,&quot;Liberation Mono&quot;,Menlo,Courier,monospace;line-height:10px;color:#555;vertical-align:middle;background-color:#fcfcfc;border:1px solid #ccc;border-bottom-color:#bbb;border-radius:3px;box-shadow:inset 0 -1px 0 #bbb;font-weight: bold} div#katalon-elementInfoDiv {color: lightblue; padding: 5px}


    
    
        
        
            
                
                      
                
                
                    Selenium Easy
                    Complete Automation Testing Tutorials
                
            
            


            
        
        
        
            
                
                    
                    
                        
                            Toggle navigation
                            
                            
                            
                        
                        Demo Home
                    

                    
                    
                        
                            
                                Input Forms
					  
                                
                                    Simple Form Demo
                                    Checkbox Demo
                                    Radio Buttons Demo
                                    Select Dropdown List
                                    Input Form Submit
                                    Ajax Form Submit
                                     JQuery Select dropdown
                                
                            
                            
                                Date pickers
					  
                                
                                    Bootstrap Date Picker
                                    JQuery Date Picker
                                
                            
                            
                                Table
					   
                                
                                    Table Pagination
                                    Table Data Search
                                    Table Filter 
                                    Table Sort &amp; Search
                                    Table Data Download
                                
                            

                        
                        
                            
                                Progress Bars
					  
                            
                                
                                    JQuery Download Progress bars
                                    Bootstrap Progress bar
                                    Drag &amp; Drop Sliders
                                
                            
                            
                                Alerts &amp; Modals
					 		
                            
                                
                                    Bootstrap Alerts
                                    Bootstrap Modals
                                    Window Popup Modal
                                    Progress Bar Modal
                                    Javascript Alerts
                                    File Download
                                
                            
                            
                                List Box
					 	
                            
                                
                                    Bootstrap List Box
                                    JQuery List Box
                                    Data List Filter
                                
                            

                              Others
                            
                            
                                
                                 Drag and Drop
                                    Dynamic Data Loading
                                    Charts Demo
                                
                            
                        
                    
                    
                
                
            
        
    
    
    
        
            
                
                    Menu List
                    
                       
                        
                            
                                All Examples
                                

                                    
                                        Input Forms
                                        
                                            Simple Form Demo
                                            Checkbox Demo
                                            Radio Buttons Demo
                                            Select Dropdown List
                                            Input Form Submit
                                            Ajax Form Submit
                                             JQuery Select dropdown
                                        
                                    
                                    
                                        Date pickers
                                        
                                            Bootstrap Date Picker
                                            JQuery Date Picker
                                        
                                    
                                    
                                        Table
                                        
                                            Table Pagination
                                            Table Data Search
                                            Table Filter 
                                            Table Sort &amp; Search
                                            Table Data Download
                                        
                                    
                                    
                                        Progress Bars &amp; Sliders
                                        
                                            JQuery Download Progress bars
                                            Bootstrap Progress bar
                                            Drag &amp; Drop Sliders
                                        
                                    
                                    
                                        Alerts &amp; Modals
                                        
                                            Bootstrap Alerts
                                            Bootstrap Modals
                                            Window Popup Modal
                                            Progress Bar Modal
                                            Javascript Alerts
                                          File Download
                                        
                                    
                                    
                                        List Box
                                        
                                            Bootstrap List Box
                                            JQuery List Box
                                            Data List Filter
                                        
                                    
                                    
                                        Others
                                        
                                        Drag and Drop
                                            Dynamic Data Loading
                                            Charts Demo
                                        
                                    
                                
                            
                        
                        
                    
                
            
            
                This would be your first example to start with Selenium.

                The tag input specifies an input field where the user can enter data.
                

                
                    Single Input Field
                    
                         First Let us try be very simple with only one input field and a Button
                        
                             Enter your message
                             Click on 'Show Message' button to display message entered in input field
                        
                        
                            
                                Enter message
                                
                            
                            Show Message
                        
                        
                            Your Message: 
                            Hello

                    
                

                
                    Two Input Fields
                    
                         First Let us try with Two input fields and a Button
                        
                             Enter Value for a 
                             Enter Value for b 
                            Click on 'Get Total' button to display the sum of two numbers 'a and b'
                        

                        
                            
                                Enter a
                                
                            
                            
                                Enter b
                                
                            
                            Get Total
                        
                        
                             Total a + b = 
                            
                        
                    
                

            

            
            
               
            
            
        
    
    
        
            
                Tutorials Menu
                
                    Selenium Tutorials
                    TestNG Tutorial
                    JUnit Tutorial
                    JXL Tutorial
                    Apache POI
                    ANT Tutorial
                    Maven Tutorial
                
            
            
                Popular Posts
                
                    Gecko Driver Selenium 3
                    Page Factory Pattern
                    Execute Tests in Jenkins 
                    TestNG Custom Report
                    Execute Selenium tests using Maven
                    View All Selenium Tutorials
                
            
        
    
    
    
    
    
        function showInput() {
            document.getElementById('display').innerHTML =
                document.getElementById(&quot;user-message&quot;).value;
        }
    
    
        function total() {
            var a = document.forms[&quot;gettotal&quot;][&quot;sum1&quot;].value;
            var b = document.forms[&quot;gettotal&quot;][&quot;sum2&quot;].value;
            //alert(a+b)
            var display = document.getElementById(&quot;displayvalue&quot;)
            display.innerHTML = parseInt(a) + parseInt(b);
            return false;
        }
    
    
        (function(i, s, o, g, r, a, m) {
            i['GoogleAnalyticsObject'] = r;
            i[r] = i[r] || function() {
                (i[r].q = i[r].q || []).push(arguments)
            }, i[r].l = 1 * new Date();
            a = s.createElement(o),
                m = s.getElementsByTagName(o)[0];
            a.async = 1;
            a.src = g;
            m.parentNode.insertBefore(a, m)
        })(window, document, 'script', 'https://www.google-analytics.com/analytics.js', 'ga');

        ga('create', 'UA-46729791-2', 'auto');
        ga('send', 'pageview');
    


/html[1]</value>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath</name>
      <type>Main</type>
      <value>/html[1]</value>
   </webElementProperties>
</WebElementEntity>
