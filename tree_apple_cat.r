#1
#Create a data frame that contains the names of entrepreneurs and their businesses
entrepreneurs <- data.frame('Name' = c('John Doe', 'Jane Doe', 'Jake Smith'), 
                            'Business' = c('Doe Industries', 'Doe Solutions', 'Smith Enterprises'))
#2
#Create a function to produce the entrepreneurial experience
entrepreneurial_experience <- function(entrepreneur_name){
  #3
  #Assign data frame columns to variables
  name <- entrepreneur_name
  business <- entrepreneurs$Business[entrepreneurs$Name == name]
  #4
  #Create message using business name for output
  message <- paste(name, 'owns and operates', business)
  #5
  #Print the message
  print(message)

  #6
  #Create a vector of success stories
  success <- c('John leveraged great customer service to increase profits',
               'Jane created a new market with a revolutionary product',
               'Jake utilized digital marketing to reach new customers')
  
  #7
  #Print a success story that is randomly chosen
  print(sample(success, 1))
}
#8
#Call the entrepreneurial experience function for all entrepreneurs in the data frame
for (name in entrepreneurs$Name){
  entrepreneurial_experience(name)
  #9
  #Print new line
  print(' ')
}
#10
#Create a customer satisfaction survey
survey <- function(business_name){
  #11
  #Assign data frame columns to variables
  name <- entrepreneurs$Name[entrepreneurs$Business == business_name]
  business <- business_name
  #12
  #Prompt customer to rate their experience
  rating <- readline(paste('Please rate your experience with', business, 'on a scale of 1-5:'))
  #13
  #Print message thanking customer for their feedback
  print(paste('Thank you,', name, 'appreciates your feedback!'))
}
#14
#Call the survey function for each entrepreneur's business
for (business in entrepreneurs$Business){
  survey(business)
  #15
  #Print new line
  print(' ')
}