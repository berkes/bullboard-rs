# frozen_string_literal: true

require_relative '../../lib/bullboard'
require 'minitest/spec'

module MinitestWorld
  extend Minitest::Assertions
end

World(MinitestWorld)
