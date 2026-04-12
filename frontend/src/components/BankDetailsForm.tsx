'use client'

import { useState } from 'react'
import { Banknote, Building2, User, ArrowRight } from 'lucide-react'

interface BankDetails {
  accountNumber: string
  bankCode: string
  accountName: string
  amount: string
}

export default function BankDetailsForm() {
  const [formData, setFormData] = useState<BankDetails>({
    accountNumber: '',
    bankCode: '',
    accountName: '',
    amount: '',
  })
  const [isSubmitting, setIsSubmitting] = useState(false)

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault()
    setIsSubmitting(true)
    
    // Placeholder for SEP-24 withdrawal flow
    console.log('Submitting withdrawal:', formData)
    
    setTimeout(() => {
      setIsSubmitting(false)
      alert('Withdrawal initiated! (Mock response)')
    }, 1500)
  }

  const handleChange = (e: React.ChangeEvent<HTMLInputElement | HTMLSelectElement>) => {
    setFormData(prev => ({
      ...prev,
      [e.target.name]: e.target.value
    }))
  }

  return (
    <div className="bg-white rounded-2xl shadow-xl p-8">
      <h2 className="text-2xl font-semibold text-gray-800 mb-6">
        Withdrawal Details
      </h2>
      
      <form onSubmit={handleSubmit} className="space-y-6">
        <div>
          <label htmlFor="amount" className="block text-sm font-medium text-gray-700 mb-2">
            <Banknote className="inline w-4 h-4 mr-2" />
            Amount (NGN)
          </label>
          <input
            type="number"
            id="amount"
            name="amount"
            value={formData.amount}
            onChange={handleChange}
            required
            min="1000"
            max="1000000"
            className="w-full px-4 py-3 border border-gray-300 rounded-lg focus:ring-2 focus:ring-indigo-500 focus:border-transparent"
            placeholder="10000"
          />
        </div>

        <div>
          <label htmlFor="bankCode" className="block text-sm font-medium text-gray-700 mb-2">
            <Building2 className="inline w-4 h-4 mr-2" />
            Bank
          </label>
          <select
            id="bankCode"
            name="bankCode"
            value={formData.bankCode}
            onChange={handleChange}
            required
            className="w-full px-4 py-3 border border-gray-300 rounded-lg focus:ring-2 focus:ring-indigo-500 focus:border-transparent"
          >
            <option value="">Select your bank</option>
            <option value="058">GTBank</option>
            <option value="044">Access Bank</option>
            <option value="033">UBA</option>
            <option value="057">Zenith Bank</option>
            <option value="011">First Bank</option>
          </select>
        </div>

        <div>
          <label htmlFor="accountNumber" className="block text-sm font-medium text-gray-700 mb-2">
            Account Number
          </label>
          <input
            type="text"
            id="accountNumber"
            name="accountNumber"
            value={formData.accountNumber}
            onChange={handleChange}
            required
            pattern="[0-9]{10}"
            maxLength={10}
            className="w-full px-4 py-3 border border-gray-300 rounded-lg focus:ring-2 focus:ring-indigo-500 focus:border-transparent"
            placeholder="0123456789"
          />
        </div>

        <div>
          <label htmlFor="accountName" className="block text-sm font-medium text-gray-700 mb-2">
            <User className="inline w-4 h-4 mr-2" />
            Account Name
          </label>
          <input
            type="text"
            id="accountName"
            name="accountName"
            value={formData.accountName}
            onChange={handleChange}
            required
            className="w-full px-4 py-3 border border-gray-300 rounded-lg focus:ring-2 focus:ring-indigo-500 focus:border-transparent"
            placeholder="John Doe"
          />
        </div>

        <button
          type="submit"
          disabled={isSubmitting}
          className="w-full bg-indigo-600 hover:bg-indigo-700 disabled:bg-gray-400 text-white font-semibold py-3 px-6 rounded-lg transition-colors flex items-center justify-center gap-2"
        >
          {isSubmitting ? 'Processing...' : 'Initiate Withdrawal'}
          <ArrowRight className="w-5 h-5" />
        </button>
      </form>

      <div className="mt-6 p-4 bg-blue-50 rounded-lg">
        <p className="text-sm text-gray-600">
          <strong>Note:</strong> Withdrawals are processed via the Stellar network. 
          Ensure your bank details are correct to avoid delays.
        </p>
      </div>
    </div>
  )
}
