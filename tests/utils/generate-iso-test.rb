#!/usr/bin/env ruby
# Usage: generate-iso-test.rb [--negative] [TEST_NAME DATASET_SIZE VAR_COUNT GROUND_COUNT]
#        Without arguments, it will generate the positive test file.
#        With --negative, it will generate the negative test file.
require 'set'

FAIL_ID = 9999

class Var
	attr_reader :id

	include Comparable

	def initialize(id)
		@id = id
	end

	def <=>(other)
		return -1 if other.class == Ground
		@id <=> other.id
	end

	def eql?(other)
		other.class == Var && @id == other.id
	end

	def hash
		@id.hash
	end

	def to_rust
		"Term::Var(#{@id})"
	end

	def substituted
		Var.new(@id + 1000)
	end
end

class Ground
	attr_reader :id

	include Comparable

	def initialize(id)
		@id = id
	end

	def <=>(other)
		return 1 if other.class == Var
		@id <=> other.id
	end

	def eql?(other)
		other.class == Ground && @id == other.id
	end

	def hash
		@id.hash
	end

	def to_rust
		"Term::Ground(#{@id})"
	end

	def substituted
		self
	end
end

class Triple
	attr_reader :subject, :predicate, :object

	def self.random_from_terms(terms)
		Triple.new(terms.sample, terms.sample, terms.sample)
	end

	def initialize(subject, predicate, object)
		@subject = subject
		@predicate = predicate
		@object = object
	end

	def eql?(other)
		other.class == Triple && @subject == other.subject && @predicate == other.predicate && @object == other.object
	end

	def hash
		@subject.hash ^ @predicate.hash ^ @object.hash
	end

	def to_s
		"#{@subject} #{@predicate} #{@object} ."
	end

	def to_rust
		"Quad(\n\t\t#{@subject.to_rust},\n\t\t#{@predicate.to_rust},\n\t\t#{@object.to_rust},\n\t\tNone,\n\t)"
	end

	def substituted
		Triple.new(@subject.substituted, @predicate.substituted, @object.substituted)
	end
end

class Dataset
	def self.random_from_terms(terms, size)
		dataset = Dataset.new
		size.times do
			while !dataset.insert(Triple.random_from_terms(terms)) do
				# nothing
			end
		end
		dataset
	end

	def initialize()
		@triples = Set.new
	end

	def insert(triple)
		!@triples.add?(triple).nil?
	end

	def to_s
		result = ""
		@triples.each do |triple|
			result += "\n" unless result.empty?
			result += triple.to_s
		end
		result
	end

	def to_rust(id)
		result = "\tlet mut #{id} = BTreeDataset::new();"
		@triples.each do |triple|
			result += "\n\t#{id}.insert(#{triple.to_rust});"
		end
		result
	end

	def substituted
		dataset = Dataset.new
		@triples.each do |triple|
			dataset.insert(triple.substituted)
		end
		dataset
	end
end

FAIL_TRIPLE = Triple.new(Ground.new(FAIL_ID), Ground.new(FAIL_ID), Ground.new(FAIL_ID))

def generate_test(name, size, var_count, ground_count, negative: false)
	vars = (0...var_count).map { |i| Var.new(i) }
	grounds = (0...ground_count).map { |i| Ground.new(i) }

	dataset = Dataset.random_from_terms(vars + grounds, size)

	prefix = negative ? "e" : ""

	puts "#[test]"
	puts "fn iso_#{prefix}#{name}() {"
	puts dataset.to_rust("a")

	b = dataset.substituted
	b.insert(FAIL_TRIPLE) if negative

	puts b.to_rust("b")
	puts "\ttest(a, b)"
	puts "}"
end

CLASSES = [
	# var count, graph size, test count
	[        1,          5,         10],
	[        2,          5,         20],
	[        3,         10,         50],
	[       10,         20,        100],
	[       50,        100,         20],
]

NEGATIVE_CLASSES = [
	# var count, graph size, test count
	[        1,          5,         10],
	[        2,         10,          5],
	[        3,         20,          4],
	[       50,        100,          1],
]

def generate_file(negative: false)
	template = negative ? "tests/utils/iso-negative.rs.template" : "tests/utils/iso.rs.template"
	classes = negative ? NEGATIVE_CLASSES : CLASSES

	puts "//! This file is auto generated using the"
	puts "//! `tests/utils/generate-iso-test.rb` script."

	puts File.open(template).read

	n = 1
	classes.each do |c|
		c[2].times do
			generate_test(n.to_s.rjust(3, '0'), c[1], c[0], c[1]-c[0], negative: negative)
			n += 1
		end
	end
end

if ARGV.include?("--negative")
	generate_file(negative: true)
else
	generate_file(negative: false)
end
