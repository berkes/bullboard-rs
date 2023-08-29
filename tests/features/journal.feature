Feature: Journal

  So that I can see my portfolio activities
  As a user
  I want to add entries to my journal
  And I want to see my journal entries

  Background:
    Given a database file to store events

  Scenario: Add a journal entry
    Given I have an empty journal
    When I add a journal entry
    Then I should see the entry in my journal

  Scenario: Save journal entries
    Given I have an empty journal
    When I add a journal entry
    And I restart the application
    Then I should see the entry in my journal
